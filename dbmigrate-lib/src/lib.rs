//! Database migrations for Postgres, MySQL, SQLite and SQLCipher.
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
#[cfg(any(feature = "sqlite_support", feature = "sqlcipher_support"))]
extern crate rusqlite as sqlite_client;
#[cfg(feature = "sqlcipher_support")]
extern crate rusqlite as sqlcipher_client;
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
