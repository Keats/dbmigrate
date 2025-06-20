use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::iter::repeat;
use std::path::Path;

use crate::errors::{Result, ResultExt};
use regex::Regex;

/// A migration direction, can be Up or Down
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    /// Migration to apply changes
    Up,
    /// Migration to rollback changes
    Down,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            Direction::Up => "up".to_owned(),
            Direction::Down => "down".to_owned(),
        }
    }
}

/// A migration represents a database change with up and optional down scripts
#[derive(Debug)]
pub struct Migration {
    /// Content of the up migration
    pub up: String,
    /// Optional content of the down migration
    pub down: Option<String>,
    /// Migration number (must be positive)
    pub number: u32,
    /// Migration name
    pub name: String,
}

/// Simple way to hold migrations indexed by their number
pub type Migrations = BTreeMap<u32, Migration>;

impl Migration {
    /// Creates a new migration
    pub fn new(up: String, down: Option<String>, number: u32, name: String) -> Self {
        Migration {
            up,
            down,
            number,
            name,
        }
    }

    /// Gets the filename for a specific direction
    pub fn get_filename(&self, direction: Direction) -> String {
        get_filename(&self.name, self.number, direction)
    }

    /// Writes the migration files to disk
    pub fn write_to_disk(&self, path: &Path) -> Result<()> {
        let filename_up = self.get_filename(Direction::Up);
        let up_path = path.join(&filename_up);

        let mut file =
            File::create(&up_path).chain_err(|| format!("Failed to create {}", filename_up))?;
        file.write_all(self.up.as_bytes())
            .chain_err(|| format!("Failed to write content to {}", filename_up))?;

        if let Some(down_content) = &self.down {
            let filename_down = self.get_filename(Direction::Down);
            let down_path = path.join(&filename_down);

            let mut file = File::create(&down_path)
                .chain_err(|| format!("Failed to create {}", filename_down))?;
            file.write_all(down_content.as_bytes())
                .chain_err(|| format!("Failed to write content to {}", filename_down))?;
        }

        Ok(())
    }
}

/// Creates a new migration with empty content
pub fn create_migration(path: &Path, slug: &str, number: u32) -> Result<()> {
    let fixed_slug = slug.replace(" ", "_");

    let migration = Migration::new(String::new(), Some(String::new()), number, fixed_slug);

    println!("Creating {}", migration.get_filename(Direction::Up));
    println!("Creating {}", migration.get_filename(Direction::Down));

    migration.write_to_disk(path)
}

/// Get the filename to use for a migration using the given data
fn get_filename(slug: &str, number: u32, direction: Direction) -> String {
    let num = number.to_string();
    let filler = repeat("0").take(4 - num.len()).collect::<String>();
    filler + &num + "." + slug + "." + &direction.to_string() + ".sql"
}

/// Information parsed from a migration filename
struct FilenameInfo {
    number: u32,
    name: String,
    direction: Direction,
}

/// Read the path given and read all the migration files, pairing them by migration
/// number and checking for errors along the way
pub fn read_migration_files(path: &Path) -> Result<Migrations> {
    let mut migrations: Migrations = BTreeMap::new();
    let mut up_files = BTreeMap::new();
    let mut down_files = BTreeMap::new();

    for entry in fs::read_dir(path).chain_err(|| format!("Failed to open {:?}", path))? {
        let entry = entry.unwrap();
        let filename = entry.file_name().to_string_lossy().to_string();

        let info = match parse_filename(&filename) {
            Ok(info) => info,
            Err(_) => continue,
        };

        let mut file =
            File::open(entry.path()).chain_err(|| format!("Failed to open {:?}", entry.path()))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        match info.direction {
            Direction::Up => {
                up_files.insert(info.number, (info.name, content));
            }
            Direction::Down => {
                down_files.insert(info.number, content);
            }
        }
    }

    for (number, (name, up_content)) in up_files {
        let down_content = down_files.remove(&number);

        let migration = Migration::new(up_content, down_content, number, name);

        migrations.insert(number, migration);
    }

    let mut expected_number = 1;
    for number in migrations.keys() {
        if *number != expected_number {
            bail!("Files for migration {} are missing", expected_number);
        }
        expected_number += 1;
    }

    if !down_files.is_empty() {
        let orphans: Vec<_> = down_files.keys().collect();
        bail!(
            "Found orphaned down migrations (no matching up file): {:?}",
            orphans
        );
    }

    Ok(migrations)
}

/// Gets a filename and check whether it's a valid format.
/// If it is, grabs all the info from it
fn parse_filename(filename: &str) -> Result<FilenameInfo> {
    let re =
        Regex::new(r"^(?P<number>[0-9]{4})\.(?P<name>[_0-9a-zA-Z]*)\.(?P<direction>up|down)\.sql$")
            .unwrap();

    let caps = match re.captures(filename) {
        None => bail!("File {} has an invalid filename", filename),
        Some(c) => c,
    };

    // Unwrapping below should be safe (in theory)
    let number = caps
        .name("number")
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();
    let name = caps.name("name").unwrap().as_str().to_string();
    let direction = if caps.name("direction").unwrap().as_str() == "up" {
        Direction::Up
    } else {
        Direction::Down
    };

    Ok(FilenameInfo {
        number,
        name,
        direction,
    })
}

#[cfg(test)]
mod tests {
    use super::{Direction, Migration, get_filename, parse_filename, read_migration_files};
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::PathBuf;
    use tempdir::TempDir;

    fn create_file(path: &PathBuf, filename: &str, content: &str) {
        let mut new_path = path.clone();
        new_path.push(filename);
        let mut f = File::create(new_path.to_str().unwrap()).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_parse_good_filename() {
        let result = parse_filename("0001.tests.up.sql").unwrap();
        assert_eq!(result.number, 1);
        assert_eq!(result.name, "tests");
        assert_eq!(result.direction, Direction::Up);
    }

    #[test]
    fn test_parse_bad_filename_format() {
        // Has _ instead of . between number and name
        let result = parse_filename("0001_tests.up.sql");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_get_filename_ok() {
        let result = get_filename("initial", 1, Direction::Up);
        assert_eq!(result, "0001.initial.up.sql");
    }

    #[test]
    fn test_migration_filename() {
        let migration = Migration::new(
            "CREATE TABLE users;".to_string(),
            Some("DROP TABLE users;".to_string()),
            1,
            "create_users".to_string(),
        );

        assert_eq!(
            migration.get_filename(Direction::Up),
            "0001.create_users.up.sql"
        );
        assert_eq!(
            migration.get_filename(Direction::Down),
            "0001.create_users.down.sql"
        );
    }

    #[test]
    fn test_parse_good_migrations_directory() {
        let pathbuf = TempDir::new("migrations").unwrap().into_path();
        create_file(&pathbuf, "0001.tests.up.sql", "CREATE TABLE tests;");
        create_file(&pathbuf, "0001.tests.down.sql", "DROP TABLE tests;");
        create_file(&pathbuf, "0002.tests_second.up.sql", "ALTER TABLE tests;");
        create_file(&pathbuf, "0002.tests_second.down.sql", "-- Revert ALTER");

        let migrations = read_migration_files(pathbuf.as_path()).unwrap();

        assert_eq!(migrations.len(), 2);

        let first = migrations.get(&1).unwrap();
        assert_eq!(first.number, 1);
        assert_eq!(first.name, "tests");
        assert_eq!(first.up, "CREATE TABLE tests;");
        assert_eq!(first.down, Some("DROP TABLE tests;".to_string()));

        let second = migrations.get(&2).unwrap();
        assert_eq!(second.number, 2);
        assert_eq!(second.name, "tests_second");
        assert_eq!(second.up, "ALTER TABLE tests;");
        assert_eq!(second.down, Some("-- Revert ALTER".to_string()));
    }

    #[test]
    fn test_migration_with_only_up() {
        let pathbuf = TempDir::new("migrations").unwrap().into_path();
        create_file(&pathbuf, "0001.tests.up.sql", "CREATE TABLE tests;");

        let migrations = read_migration_files(pathbuf.as_path()).unwrap();
        assert_eq!(migrations.len(), 1);

        let first = migrations.get(&1).unwrap();
        assert_eq!(first.number, 1);
        assert_eq!(first.name, "tests");
        assert_eq!(first.up, "CREATE TABLE tests;");
        assert_eq!(first.down, None);
    }

    #[test]
    fn test_parse_skipping_migrations_directory() {
        let pathbuf = TempDir::new("migrations").unwrap().into_path();
        create_file(&pathbuf, "0001.tests.up.sql", "CREATE TABLE tests;");
        create_file(&pathbuf, "0001.tests.down.sql", "DROP TABLE tests;");
        create_file(&pathbuf, "0003.tests_second.up.sql", "ALTER TABLE tests;");
        create_file(&pathbuf, "0003.tests_second.down.sql", "-- Revert ALTER");

        let migrations = read_migration_files(pathbuf.as_path());
        assert!(migrations.is_err());
    }

    #[test]
    fn test_orphaned_down_migration() {
        let pathbuf = TempDir::new("migrations").unwrap().into_path();
        create_file(&pathbuf, "0001.tests.up.sql", "CREATE TABLE tests;");
        create_file(&pathbuf, "0001.tests.down.sql", "DROP TABLE tests;");
        create_file(&pathbuf, "0002.orphaned.down.sql", "Something wrong");

        let migrations = read_migration_files(pathbuf.as_path());
        assert!(migrations.is_err());
    }

    #[test]
    fn test_create_and_write_migration() {
        let dir = TempDir::new("migrations").unwrap();
        let path = dir.path();

        let migration = Migration::new(
            "CREATE TABLE users;".to_string(),
            Some("DROP TABLE users;".to_string()),
            1,
            "create_users".to_string(),
        );

        migration.write_to_disk(path).unwrap();

        let mut up_file = File::open(path.join("0001.create_users.up.sql")).unwrap();
        let mut up_content = String::new();
        up_file.read_to_string(&mut up_content).unwrap();
        assert_eq!(up_content, "CREATE TABLE users;");

        let mut down_file = File::open(path.join("0001.create_users.down.sql")).unwrap();
        let mut down_content = String::new();
        down_file.read_to_string(&mut down_content).unwrap();
        assert_eq!(down_content, "DROP TABLE users;");
    }
}
