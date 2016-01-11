use std::path::Path;

use time::PreciseTime;

use drivers::Driver;
use files::{create_migration, Migrations, Direction};
use print;


// Does the whole migration thingy, along with timing and handling errors
macro_rules! migrate {
    ($driver: ident, $mig_file: ident) => {
        println!(
            "Running {} migration #{}: {}",
            $mig_file.direction.to_string(), $mig_file.number, $mig_file.name
        );
        {
            let start = PreciseTime::now();

            match $driver.migrate(
                $mig_file.content.clone().unwrap(),
                if $mig_file.direction == Direction::Up { $mig_file.number } else { $mig_file.number - 1}
            ) {
                Err(e) => e.exit(),
                Ok(_) => {
                    let duration = start.to(PreciseTime::now());
                    print::success(&format!("> Done in {} second(s)", duration.num_seconds()));
                }
            }
        }
    }
}

pub fn create(migration_files: &Migrations, path: &Path, slug: &str) {
    let current_number = migration_files.keys().cloned().max().unwrap_or(0i32);
    let number = current_number + 1;
    match create_migration(path, slug, number) {
        Err(e) => e.exit(),
        Ok(_) => {
            print::success("Migration files successfully created!");
        }
    }
}


pub fn status(driver: Box<Driver>, migration_files: &Migrations) {
    let current = driver.get_current_number();
    if current == 0 {
        print::success("No migration has been ran");
    }
    for (number, migration) in migration_files.iter(){
        let mig_file = migration.up.as_ref().unwrap();
        if number == &current {
            print::success(&format!("{} - {} (current)", mig_file.number, mig_file.name));
        } else {
            println!("{} - {}", mig_file.number, mig_file.name);
        }
    }
}


pub fn up(driver: Box<Driver>, migration_files: &Migrations) {
    let current = driver.get_current_number();
    let max = migration_files.keys().max().unwrap();
    if current == *max {
        print::success("Migrations are up-to-date");
        return;
    }

    for (number, migration) in migration_files.iter(){
        if number > &current {
            let mig_file = migration.up.as_ref().unwrap();
            migrate!(driver, mig_file);
        }
    }
}

pub fn down(driver: Box<Driver>, migration_files: &Migrations) {
    let current = driver.get_current_number();
    if current == 0 {
        print::success("No down migrations to run");
        return;
    }

    let mut numbers: Vec<i32> = migration_files.keys().cloned().filter(|i| i <= &current).collect();
    numbers.sort_by(|a, b| b.cmp(a));

    for number in numbers {
        let migration = migration_files.get(&number).unwrap();
        let mig_file = migration.down.as_ref().unwrap();
        migrate!(driver, mig_file);
    }
}

pub fn redo(driver: Box<Driver>, migration_files: &Migrations) {
    let current = driver.get_current_number();
    if current == 0 {
        print::success("No migration to redo");
        return;
    }
    let migration = migration_files.get(&current).unwrap();

    let down_file = migration.down.as_ref().unwrap();
    let up_file = migration.up.as_ref().unwrap();

    migrate!(driver, down_file);
    migrate!(driver, up_file);
}


pub fn revert(driver: Box<Driver>, migration_files: &Migrations) {
    let current = driver.get_current_number();
    if current == 0 {
        print::success("No migration to revert");
        return;
    }
    let migration = migration_files.get(&current).unwrap();
    let down_file = migration.down.as_ref().unwrap();

    migrate!(driver, down_file);
}
