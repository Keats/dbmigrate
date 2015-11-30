use std::fs;
use std::fs::File;
use std::io::Read;
use std::iter::{repeat};
use std::path::Path;
use std::collections::{BTreeMap};

use regex::Regex;
use errors::{invalid_filename, migration_skipped, missing_file, MigrateResult};


#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            Direction::Up => "up".to_owned(),
            Direction::Down => "down".to_owned()
        }
    }
}

#[derive(Debug)]
pub struct MigrationFile {
    pub content: Option<String>,
    pub direction: Direction,
    pub number: i32,
    pub filename: String,
    pub name: String
}

impl MigrationFile {
    /// Used when getting the info, therefore setting content to None at that point
    fn new(filename: &str, name: &str, number: i32, direction: Direction) -> MigrationFile {
        MigrationFile {
            content: None,
            filename: filename.to_owned(),
            number: number,
            name: name.to_owned(),
            direction: direction
        }
    }
}

#[derive(Debug)]
pub struct Migration {
    up: Option<MigrationFile>,
    down: Option<MigrationFile>
}

/// Creates 2 migration file: one up and one down
pub fn create_migration(path: &Path, slug: &str, number: i32) -> MigrateResult<()> {
    let filename_up = get_filename(slug, number, Direction::Up);
    let filename_down = get_filename(slug, number, Direction::Down);

    try!(File::create(path.join(filename_up)));
    try!(File::create(path.join(filename_down)));

    Ok(())
}

/// Get the filename to use for a migration using the given data
fn get_filename(slug: &str, number: i32, direction: Direction) -> String {
    let num = number.to_string();
    let filler = repeat("0").take(4 - num.len()).collect::<String>();
    filler + &num + "." + slug + "." + &direction.to_string() + ".sql"
}

pub type Migrations = BTreeMap<i32, Migration>;

/// Read the path given and read all the migration files, pairing them by migration
/// number and checking for errors along the way
pub fn read_migrations_files(path: &Path) -> MigrateResult<Migrations> {
    let mut btreemap: Migrations = BTreeMap::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        // Will panic on invalid unicode in filename, unlikely (heh)
        let info = try!(parse_filename(entry.file_name().to_str().unwrap()));
        let mut file = try!(File::open(entry.path()));
        let mut content = String::new();
        try!(file.read_to_string(&mut content));

        let migration_file = MigrationFile {content: Some(content), ..info};
        let migration_number = migration_file.number;
        let mut migration = match btreemap.remove(&migration_number) {
            None => Migration {up: None, down: None},
            Some(m) => m
        };
        if migration_file.direction == Direction::Up {
            migration.up = Some(migration_file);
        } else {
            migration.down = Some(migration_file);
        }
        btreemap.insert(migration_number, migration);
    }

    // Let's check the all the files we need now
    let mut index = 1;
    for (number, migration) in btreemap.iter() {
        if index != *number {
            return Err(migration_skipped(index));
        }
        if migration.up.is_none() || migration.down.is_none() {
            return Err(missing_file(index));
        }
        index += 1;
    }
    Ok(btreemap)
}

/// Gets a filename and check whether it's a valid format.
/// If it is, grabs all the info from it
fn parse_filename(filename: &str) -> MigrateResult<MigrationFile> {
    let re = Regex::new(
        r"^(?P<number>[0-9]{4})\.(?P<name>[_0-9a-zA-Z]*)\.(?P<direction>up|down)\.sql$"
    ).unwrap();

    let caps = match re.captures(filename) {
        None => return Err(invalid_filename(filename)),
        Some(c) => c
    };

    // Unwrapping below should be safe (in theory)
    let number = caps.name("number").unwrap().parse::<i32>().unwrap();
    let name = caps.name("name").unwrap();
    let direction = if caps.name("direction").unwrap() == "up" {
        Direction::Up
    } else {
        Direction::Down
    };

    Ok(MigrationFile::new(filename, name, number, direction))
}

#[cfg(test)]
mod tests {
    use super::{parse_filename, read_migrations_files, Direction, get_filename};
    use tempdir::TempDir;
    use std::path::{PathBuf};
    use std::io::prelude::*;
    use std::fs::File;

    fn create_file(path: &PathBuf, filename: &str) {
        let mut new_path = path.clone();
        new_path.push(filename);
        let mut f = File::create(new_path.to_str().unwrap()).unwrap();
        f.write_all(b"Hello, world!").unwrap();
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
        let result = parse_filename("0001_tests.up.sql");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_get_filename_ok() {
        let result = get_filename("initial", 1, Direction::Up);
        assert_eq!(result, "0001.initial.up.sql");
    }

    #[test]
    fn test_parse_good_migrations_directory() {
        let pathbuf = TempDir::new("migrations").unwrap().into_path();
        create_file(&pathbuf, "0001.tests.up.sql");
        create_file(&pathbuf, "0001.tests.down.sql");
        create_file(&pathbuf, "0002.tests_second.up.sql");
        create_file(&pathbuf, "0002.tests_second.down.sql");
        let migrations = read_migrations_files(pathbuf.as_path());

        assert_eq!(migrations.is_ok(), true);
    }

    #[test]
    fn test_parse_missing_migrations_directory() {
        let pathbuf = TempDir::new("migrations").unwrap().into_path();
        create_file(&pathbuf, "0001.tests.up.sql");
        create_file(&pathbuf, "0001.tests.down.sql");
        create_file(&pathbuf, "0002.tests_second.up.sql");
        let migrations = read_migrations_files(pathbuf.as_path());

        assert_eq!(migrations.is_err(), true);
    }

    #[test]
    fn test_parse_skipping_migrations_directory() {
        let pathbuf = TempDir::new("migrations").unwrap().into_path();
        create_file(&pathbuf, "0001.tests.up.sql");
        create_file(&pathbuf, "0001.tests.down.sql");
        create_file(&pathbuf, "0003.tests_second.up.sql");
        create_file(&pathbuf, "0003.tests_second.down.sql");
        let migrations = read_migrations_files(pathbuf.as_path());

        assert_eq!(migrations.is_err(), true);
    }
}
