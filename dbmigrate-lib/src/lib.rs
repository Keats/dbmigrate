//! Database migrations for Postgres, MySQL, and SQLite.
//!
#![deny(missing_docs)]

#[cfg(test)]
extern crate tempdir;

#[cfg(feature = "mysql_support")]
extern crate mysql as mysql_client;
#[cfg(feature = "postgres_support")]
extern crate native_tls;
#[cfg(feature = "postgres_support")]
extern crate postgres as postgres_client;
#[cfg(feature = "postgres_support")]
extern crate postgres_native_tls;
extern crate regex;
#[cfg(feature = "sqlite_support")]
extern crate rusqlite as sqlite_client;
#[cfg(feature = "surreal_support")]
extern crate surrealdb as surreal_client;
#[cfg(feature = "surreal_support")]
extern crate tokio;
extern crate url;
#[macro_use]
extern crate error_chain;

mod drivers;
/// All possible errors
pub mod errors;
mod files;

#[cfg(feature = "mysql_support")]
pub use drivers::mysql::Mysql as MysqlDriver;
#[cfg(feature = "postgres_support")]
pub use drivers::postgres::Postgres as PostgresDriver;
#[cfg(feature = "sqlite_support")]
pub use drivers::sqlite::Sqlite as SqliteDriver;
pub use drivers::{Driver, get_driver};

pub use files::{Direction, Migration, Migrations, create_migration, read_migration_files};
