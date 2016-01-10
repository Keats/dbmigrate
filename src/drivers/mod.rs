///! Driver interface and implementations
use errors::{MigrateResult, invalid_url};


mod postgres;

pub trait Driver {
    fn ensure_migration_table_exists(&self);
    fn remove_migration_table(&self);
    fn get_current_number(&self) -> i32;
    fn set_current_number(&self, number: i32);
    fn migrate(&self, migration: String, number: i32) -> MigrateResult<()>;
}


/// Returns a driver instance depending on url
pub fn get_driver(url: &str) -> MigrateResult<Box<Driver>> {
    match url.split(":").collect::<Vec<_>>()[0] {
        "postgres" => postgres::Postgres::new(url).map(|d| Box::new(d) as Box<Driver>),
        _ => Err(invalid_url(url))
    }
}

