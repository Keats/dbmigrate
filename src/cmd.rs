use std::path::Path;

use drivers::pg::Postgres;
use drivers::Driver;
use files::{create_migration, Migrations};


pub fn create(migration_files: &Migrations, path: &Path, slug: &str) {
    // should be safe
    let current_number = migration_files.keys().max().unwrap();
    let number = current_number + 1;

    match create_migration(path, slug, number) {
        Err(e) => e.exit(),
        Ok(_) => {
            println!("Migration files successfully created!");
        }
    }
}


pub fn status(url: &str, migration_files: &Migrations) {
    let pg = Postgres::new(url).unwrap_or_else(|e| e.exit());
    let current = pg.get_current_number();
    for (number, migration) in migration_files.iter(){
        let mig_file = migration.up.as_ref().unwrap();
        println!("{} - {}", mig_file.number, mig_file.name);
    }
    println!("{:?}", current);
}


pub fn up(url: &str, migration_files: &Migrations) {
    let pg = Postgres::new(url).unwrap_or_else(|e| e.exit());
    let current = pg.get_current_number();
    for (number, migration) in migration_files.iter(){
        if number > &current {
            let mig_file = migration.up.as_ref().unwrap();
            let content = mig_file.content.clone().unwrap();
            pg.migrate(content, mig_file.number);
            println!("Ran migration #{}: {}", mig_file.number, mig_file.name);
        }
    }
}

pub fn down(url: &str, migration_files: &Migrations) {
    let pg = Postgres::new(url).unwrap_or_else(|e| e.exit());
    let current = pg.get_current_number();
    let mut numbers: Vec<i32> = migration_files.keys().cloned().filter(|i| i <= &current).collect();
    numbers.sort_by(|a, b| b.cmp(a));

    for number in numbers {
        let migration = migration_files.get(&number).unwrap();
        let mig_file = migration.down.as_ref().unwrap();
        let content = mig_file.content.clone().unwrap();
        pg.migrate(content, mig_file.number - 1);
        println!("Ran down migration #{}: {}", mig_file.number, mig_file.name);

    }
}
