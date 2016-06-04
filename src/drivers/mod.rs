///! Driver interface and implementations
use url::{Url};

use errors::{MigrateResult, invalid_url};

mod mysql;
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
    let parsed_url = try!(Url::parse(url));
    match parsed_url.scheme() {
        "postgres" => postgres::Postgres::new(url).map(|d| Box::new(d) as Box<Driver>),
        "mysql" => mysql::Mysql::new(url).map(|d| Box::new(d) as Box<Driver>),
        _ => Err(invalid_url(url))
    }
}

