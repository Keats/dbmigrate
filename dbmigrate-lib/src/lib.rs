//! Database migrations for Postgres, MySQL, SQLite and Cassandra.
//!

#[cfg(test)]
extern crate tempdir;

extern crate regex;
extern crate url;
extern crate postgres as postgres_client;
extern crate mysql as mysql_client;
extern crate rusqlite as sqlite_client;
#[macro_use]
extern crate error_chain;

pub mod files;
pub mod drivers;
pub mod errors;
