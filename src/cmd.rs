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