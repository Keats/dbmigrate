use std::path::Path;
use std::time::Instant;

use dbmigrate_lib::{create_migration, Direction, Driver, Migrations};
use errors::Result;
use print;

// Does the whole migration thingy, along with timing and handling errors
macro_rules! migrate {
    ($driver: ident, $migration_file: ident, $direction: expr) => {
        println!(
            "Running {} migration #{}: {}",
            $direction.to_string(),
            $migration_file.number,
            $migration_file.name
        );
        let res = {
            let start = Instant::now();

            let content = match $direction {
                Direction::Up => $migration_file.up.clone(),
                Direction::Down => $migration_file
                    .down
                    .clone()
                    .expect("Down migration content missing"),
            };
            let index = if $direction == Direction::Up {
                $migration_file.number
            } else {
                ($migration_file.number - 1)
            };

            match $driver.migrate(content, index) {
                Err(e) => Err(e),
                Ok(_) => {
                    let duration = start.elapsed();
                    print::success(&format!("> Done in {} second(s)", duration.as_secs()));
                    Ok(())
                }
            }
        };
        if res.is_err() {
            return res.map_err(|e| e.into());
        }
    };
}

pub fn create(migration_files: &Migrations, path: &Path, slug: &str) -> Result<()> {
    let current_number = migration_files.keys().cloned().max().unwrap_or(0u32);
    let number = current_number + 1;
    match create_migration(path, slug, number) {
        Err(e) => Err(e.into()),
        Ok(_) => {
            print::success("Migration files successfully created!");
            Ok(())
        }
    }
}

pub fn status(mut driver: Box<dyn Driver>, migration_files: &Migrations) -> Result<()> {
    let current = driver.get_current_number().max(0);
    if current == 0 {
        print::success("No migration has been ran");
    }
    for migration in migration_files.values() {
        if migration.number == current {
            print::success(&format!(
                "{} - {} (current)",
                migration.number, migration.name
            ));
        } else {
            println!("{} - {}", migration.number, migration.name);
        }
    }
    Ok(())
}

pub fn up(mut driver: Box<dyn Driver>, migration_files: &Migrations) -> Result<()> {
    let current = driver.get_current_number().max(0);
    let max = migration_files.keys().cloned().max().unwrap_or(0);
    if current == max {
        print::success("Migrations are up-to-date");
        return Ok(());
    }

    for migration in migration_files.values() {
        if migration.number > current {
            migrate!(driver, migration, Direction::Up);
        }
    }
    Ok(())
}

pub fn down(mut driver: Box<dyn Driver>, migration_files: &Migrations) -> Result<()> {
    let current = driver.get_current_number().max(0);
    if current == 0 {
        print::success("No down migrations to run");
        return Ok(());
    }

    let mut numbers: Vec<u32> = migration_files
        .keys()
        .cloned()
        .filter(|i| i <= &current)
        .collect();
    numbers.sort_by(|a, b| b.cmp(a));

    for number in numbers {
        let migration = &migration_files[&number];
        migrate!(driver, migration, Direction::Down);
    }
    Ok(())
}

pub fn redo(mut driver: Box<dyn Driver>, migration_files: &Migrations) -> Result<()> {
    let current = driver.get_current_number().max(0);
    if current == 0 {
        print::success("No migration to redo");
        return Ok(());
    }
    let migration = &migration_files[&current];

    migrate!(driver, migration, Direction::Down);
    migrate!(driver, migration, Direction::Up);
    Ok(())
}

pub fn revert(mut driver: Box<dyn Driver>, migration_files: &Migrations) -> Result<()> {
    let current = driver.get_current_number().max(0);
    if current == 0 {
        print::success("No migration to revert");
        return Ok(());
    }
    let migration = &migration_files[&current];

    migrate!(driver, migration, Direction::Down);
    Ok(())
}
