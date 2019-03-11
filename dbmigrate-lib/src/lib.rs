//! Database migrations for Postgres, MySQL, and SQLite.
//!
#![deny(missing_docs)]

#[cfg(test)]
extern crate tempdir;

extern crate regex;
extern crate url;
#[cfg(feature = "postgres_support")]
extern crate postgres as postgres_client;
#[cfg(feature = "postgres_support")]
extern crate postgres_native_tls;
#[cfg(feature = "mysql_support")]
extern crate mysql as mysql_client;
#[cfg(feature = "sqlite_support")]
extern crate rusqlite as sqlite_client;
#[macro_use]
extern crate error_chain;

mod files;
mod drivers;
/// All possible errors
pub mod errors;

pub use drivers::{get_driver, Driver};
pub use files::{
    create_migration,
    read_migration_files,
    MigrationFile,
    Migration,
    Migrations,
    Direction,
};
