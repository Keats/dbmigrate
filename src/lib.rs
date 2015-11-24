//! Handles Postgres migrations
//!

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[cfg(test)]
extern crate tempdir;

extern crate regex;
extern crate postgres;


mod files;
mod drivers;
mod errors;
