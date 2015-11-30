use std::path::Path;

use drivers::pg::Postgres;
use drivers::Driver;
use files::{read_migrations_files, create_migration, Migrations};


pub fn create(url: &str, migration_files: Migrations, path: &Path, slug: &str) {
    let pg = Postgres::new(url).unwrap();
    let number = pg.get_current_number() + 1;
    // File already exists!
    if migration_files.contains_key(&number) {
        println!("Migration files for number {:?} already exist.", number);
        return;
    }

    match create_migration(path, slug, number) {
        Err(e) => {
            println!("Errored creating migration files");
            println!("{:?}", e);
        }
        Ok(_) => {
            println!("Migration file created!");
        }
    }
}


pub fn status(url: &str, path: &str) {
    // TODO: make generic
    let pg = Postgres::new(url).unwrap();
    let migration_files = read_migrations_files(Path::new(path));
    let current = pg.get_current_number();
    println!("{:?}", current);
}
