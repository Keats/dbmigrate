///! Driver interface and implementations
use url::{Url};

use errors::{Result, ResultExt};

#[cfg(feature = "mysql_support")]
mod mysql;
#[cfg(feature = "postgres_support")]
mod postgres;
#[cfg(feature = "sqlite_support")]
mod sqlite;


/// The common trait that all databases need to implement in order
/// for migrations to work
pub trait Driver {
    /// A fn that will create a migration table if it doesn't exist
    /// Otherwise do nothing
    fn ensure_migration_table_exists(&self);
    /// A fn that will delete migration table
    fn remove_migration_table(&self);
    /// Get the current migration number from the database
    fn get_current_number(&self) -> i32;
    /// Set the current migration number in the database
    fn set_current_number(&self, number: i32);
    /// Perform the `migration` content on the database and set
    /// the migration number to be the `number` given
    fn migrate(&self, migration: String, number: i32) -> Result<()>;
}

/// Returns a driver instance depending on url
pub fn get_driver(url: &str) -> Result<Box<Driver>> {
    let parsed_url = Url::parse(url)
        .chain_err(|| format!("Invalid URL: {}", url))?;

    match parsed_url.scheme() {
        #[cfg(feature = "postgres_support")]
        "postgres" => postgres::Postgres::new(url).map(|d| Box::new(d) as Box<Driver>),
        #[cfg(feature = "mysql_support")]
        "mysql" => mysql::Mysql::new(url).map(|d| Box::new(d) as Box<Driver>),
        #[cfg(feature = "sqlite_support")]
        "sqlite" => sqlite::Sqlite::new(url).map(|d| Box::new(d) as Box<Driver>),
        _ => bail!("Invalid URL: {}", url)
    }
}

