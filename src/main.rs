//! Handles Postgres migrations
//!

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[cfg(test)]
extern crate tempdir;

#[macro_use]
extern crate clap;
extern crate regex;
extern crate url;
extern crate postgres as postgres_client;
extern crate mysql as mysql_client;
extern crate time;
extern crate term;
extern crate openssl;

use std::path::Path;
use std::env;

use time::PreciseTime;

mod files;
mod drivers;
mod errors;
mod cmd;
mod print;


fn main() {
    let matches = clap_app!(myapp =>
        (@setting SubcommandRequiredElseHelp)
        (version: &crate_version!()[..])
        (author: "Vincent Prouillet <vincent@wearewizards.io>")
        (about: "
Handles migrations for databases.
Each call requires the database url and the path to the directory containing
the SQL migration files.
Those can be set using the DBMIGRATE_URL and DBMIGRATE_PATH environment variables
or the --url and --path arguments.
Using arguments will override the environment variables.
        ")
        (@arg url: -u --url +takes_value "Sets the URL of the database to use.")
        (@arg path: -p --path +takes_value "Sets the folder containing the migrations")
        (@subcommand create =>
            (about: "Creates two migration files (up and down) with the given slug")
            (@arg slug: +required "Sets the name of the migration. `.` (dot) is not allowed in the name")
        )
        (@subcommand status =>
            (about: "See list of migrations and which ones are applied")
        )
        (@subcommand up =>
            (about: "Apply all non-applied migrations")
        )
        (@subcommand down =>
            (about: "Un-apply all applied migrations")
        )
        (@subcommand redo =>
            (about: "Rollback the current migration and re-run it")
        )
        (@subcommand revert =>
            (about: "Revert the current migration")
        )
    ).get_matches();


    let url = matches.value_of("url")
        .map(|s| s.into())
        .or(env::var("DBMIGRATE_URL").ok())
        .unwrap_or_else(|| errors::no_database_url().exit());
    let driver = drivers::get_driver(&url).unwrap_or_else(|e| e.exit());

    let path_value = matches.value_of("path")
        .map(|s| s.into())
        .or(env::var("DBMIGRATE_PATH").ok())
        .unwrap_or_else(|| errors::no_migration_path().exit());
    let path = Path::new(&path_value);

    let migration_files = files::read_migrations_files(path)
        .unwrap_or_else(|e| e.exit());

    let start = PreciseTime::now();

    match matches.subcommand_name() {
        Some("status") => cmd::status(driver, &migration_files),
        Some("create") => {
            // Should be safe unwraps
            let slug = matches.subcommand_matches("create").unwrap().value_of("slug").unwrap();
            cmd::create(&migration_files, path, slug)
        },
        Some("up") => cmd::up(driver, &migration_files),
        Some("down") => cmd::down(driver, &migration_files),
        Some("redo") => cmd::redo(driver, &migration_files),
        Some("revert") => cmd::revert(driver, &migration_files),
        None        => println!("No subcommand was used"),
        _           => println!("Some other subcommand was used"),
    }

    let duration = start.to(PreciseTime::now());
    let minutes = duration.num_minutes();
    let seconds = duration.num_seconds();
    // Spacing
    println!("");
    if minutes == 0 && seconds == 0 {
        println!("Operation took less than 1 second");
    } else {
        println!("Operation took {} minutes and {} seconds", minutes, seconds);
    }
}
