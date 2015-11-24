use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::{BTreeMap};

use regex::Regex;
use errors::{LibError, MigrateResult};


#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down
}


#[derive(Debug)]
struct MigrationFile {
    content: Option<String>,
    filename: String,
    number: u8,
    name: String,
    direction: Direction
}

impl MigrationFile {
    /// Used when getting the info, therefore setting content to None at that point
    fn new(filename: &str, name: &str, number: u8, direction: Direction) -> MigrationFile {
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


/// Read the path given and read all the migration files, pairing them by migration
/// number and checking for errors along the way
pub fn read_migrations_files(path: &Path) -> MigrateResult<BTreeMap<u8, Migration>> {
    let mut btreemap: BTreeMap<u8, Migration> = BTreeMap::new();

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
            // TODO: add the number of the missing migration
            return Err(LibError::MigrationSkipped);
        }
        if migration.up.is_none() || migration.down.is_none() {
            return Err(LibError::MissingFile);
        }
        index += 1;
    }
    Ok(btreemap)
}

/// Gets a filename and check whether it's a valid format.
/// If it is, grabs all the info from it
fn parse_filename(filename: &str) -> MigrateResult<MigrationFile> {
    let re = Regex::new(
        r"^(?P<number>[0-9]{4})_(?P<name>[_0-9a-zA-Z]*)\.(?P<direction>up|down)\.sql$"
    ).unwrap();

    let caps = match re.captures(filename) {
        None => return Err(LibError::InvalidFilename),
        Some(c) => c
    };

    // Unwrapping below should be safe (in theory)
    let number = caps.name("number").unwrap().parse::<u8>().unwrap();
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
    use super::{parse_filename, read_migrations_files, Direction};
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
        let result = parse_filename("0001_tests.up.sql").unwrap();
        assert_eq!(result.number, 1);
        assert_eq!(result.name, "tests");
        assert_eq!(result.direction, Direction::Up);
    }

    #[test]
    fn test_parse_bad_filename_format() {
        let result = parse_filename("0001.tests.up.sql");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_good_migrations_directory() {
        let pathbuf = TempDir::new("migrations").unwrap().into_path();
        create_file(&pathbuf, "0001_tests.up.sql");
        create_file(&pathbuf, "0001_tests.down.sql");
        create_file(&pathbuf, "0002_tests_second.up.sql");
        create_file(&pathbuf, "0002_tests_second.down.sql");
        let migrations = read_migrations_files(pathbuf.as_path());

        assert_eq!(migrations.is_ok(), true);
    }

    #[test]
    fn test_parse_missing_migrations_directory() {
        let pathbuf = TempDir::new("migrations").unwrap().into_path();
        create_file(&pathbuf, "0001_tests.up.sql");
        create_file(&pathbuf, "0001_tests.down.sql");
        create_file(&pathbuf, "0002_tests_second.up.sql");
        let migrations = read_migrations_files(pathbuf.as_path());

        assert_eq!(migrations.is_err(), true);
    }

    #[test]
    fn test_parse_skipping_migrations_directory() {
        let pathbuf = TempDir::new("migrations").unwrap().into_path();
        create_file(&pathbuf, "0001_tests.up.sql");
        create_file(&pathbuf, "0001_tests.down.sql");
        create_file(&pathbuf, "0003_tests_second.up.sql");
        create_file(&pathbuf, "0003_tests_second.down.sql");
        let migrations = read_migrations_files(pathbuf.as_path());

        assert_eq!(migrations.is_err(), true);
    }
}
