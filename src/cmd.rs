use std::path::Path;

use drivers::pg::Postgres;
use drivers::Driver;
use files::{create_migration, Migrations};
use print;


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


pub fn status(url: &str, migration_files: &Migrations) {
    let pg = Postgres::new(url).unwrap_or_else(|e| e.exit());
    let current = pg.get_current_number();
    for (number, migration) in migration_files.iter(){
        let mig_file = migration.up.as_ref().unwrap();
        if number == &current {
            print::success(&format!("{} - {} (current)", mig_file.number, mig_file.name));
        } else {
            println!("{} - {}", mig_file.number, mig_file.name);
        }
    }
}


pub fn up(url: &str, migration_files: &Migrations) {
    let pg = Postgres::new(url).unwrap_or_else(|e| e.exit());
    let current = pg.get_current_number();
    let max = migration_files.keys().max().unwrap();
    if current == *max {
        print::success("Migrations are up-to-date");
        return;
    }

    for (number, migration) in migration_files.iter(){
        if number > &current {
            let mig_file = migration.up.as_ref().unwrap();
            let content = mig_file.content.clone().unwrap();
            println!("Running up migration #{}: {}", mig_file.number, mig_file.name);
            match pg.migrate(content, mig_file.number) {
                Err(e) => e.exit(),
                Ok(_) => {}
            }
        }
    }
}

pub fn down(url: &str, migration_files: &Migrations) {
    let pg = Postgres::new(url).unwrap_or_else(|e| e.exit());
    let current = pg.get_current_number();
    if current == 0 {
        print::success("No down migrations to run");
        return;
    }

    let mut numbers: Vec<i32> = migration_files.keys().cloned().filter(|i| i <= &current).collect();
    numbers.sort_by(|a, b| b.cmp(a));

    for number in numbers {
        let migration = migration_files.get(&number).unwrap();
        let mig_file = migration.down.as_ref().unwrap();
        let content = mig_file.content.clone().unwrap();
        println!("Running down migration #{}: {}", mig_file.number, mig_file.name);
        match pg.migrate(content, mig_file.number - 1) {
            Err(e) => e.exit(),
            Ok(_) => {}
        }
    }
}

pub fn redo(url: &str, migration_files: &Migrations) {
    let pg = Postgres::new(url).unwrap_or_else(|e| e.exit());
    let current = pg.get_current_number();
    if current == 0 {
        print::success("No migration to redo");
        return;
    }
    let migration = migration_files.get(&current).unwrap();

    let down_file = migration.down.as_ref().unwrap();
    let up_file = migration.up.as_ref().unwrap();

    println!("Running down migration #{}: {}", current, down_file.name);
    match pg.migrate(down_file.content.clone().unwrap(), current - 1) {
        Err(e) => e.exit(),
        Ok(_) => {}
    }
    println!("Running up migration #{}: {}", current, up_file.name);
    match pg.migrate(up_file.content.clone().unwrap(), current) {
        Err(e) => e.exit(),
        Ok(_) => {}
    }
}
