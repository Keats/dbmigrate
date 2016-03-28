///! Driver interface and implementations
use url::{SchemeType, UrlParser};

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


// Creating our own scheme mapper with the default ports
fn db_scheme_type_mapper(scheme: &str) -> SchemeType {
    match scheme {
        "postgres" => SchemeType::Relative(5432),
        "mysql" => SchemeType::Relative(3306),
        _ => SchemeType::NonRelative,
    }
}

/// Returns a driver instance depending on url
pub fn get_driver(url: &str) -> MigrateResult<Box<Driver>> {
    // Mysql driver does not allow to connect using a url so we need to parse it
    let mut url_parser = UrlParser::new();
    url_parser.scheme_type_mapper(db_scheme_type_mapper);
    let parsed = url_parser.parse(url).unwrap();

    match parsed.scheme.as_ref() {
        "postgres" => postgres::Postgres::new(url).map(|d| Box::new(d) as Box<Driver>),
        "mysql" => mysql::Mysql::new(parsed).map(|d| Box::new(d) as Box<Driver>),
        _ => Err(invalid_url(url))
    }
}

