//! CLI to manage SQL migrations for Postgres, MySQL and SQLite
//!

#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
extern crate dbmigrate_lib;
extern crate term;
extern crate dotenv;

use std::path::Path;
use std::env;
use std::time::Instant;

mod cmd;
mod print;
mod errors;

use errors::{Result, ResultExt};
use dbmigrate_lib::files::read_migration_files;
use dbmigrate_lib::drivers::get_driver;


fn main() {
    if let Err(ref e) = run() {
        print::error(&format!("{}", e));
        for e in e.iter().skip(1) {
            print::error(&format!("caused by: {}", e));
        }

        ::std::process::exit(1);
    }
}


fn run() -> Result<()> {
    dotenv::dotenv().ok();

    let matches = clap_app!(myapp =>
        (@setting SubcommandRequiredElseHelp)
        (version: &crate_version!()[..])
        (author: "Vincent Prouillet <vincent@wearewizards.io>")
        (about: "
Handles migrations for databases.
Each call requires the database url and the path to the directory containing
the SQL migration files.
Those can be set using the DBMIGRATE_URL and DBMIGRATE_PATH environment
variables, via a .env file, or the --url and --path arguments.
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

    let path_value = match matches.value_of("path").map(|s| s.into()).or(env::var("DBMIGRATE_PATH").ok()) {
      Some(u) => u,
      None => bail!("No migration path was provided in the environment or via a command arg.")
    };
    let path = Path::new(&path_value);

    let migration_files = read_migration_files(path)?;

    if let Some("create") = matches.subcommand_name() {
        // Should be safe unwraps
        let slug = matches.subcommand_matches("create").unwrap().value_of("slug").unwrap();
        match cmd::create(&migration_files, path, slug) {
            Ok(_) => std::process::exit(0),
            Err(e) => return Err(e)
        }
    }

    let url = match matches.value_of("url").map(|s| s.into()).or(env::var("DBMIGRATE_URL").ok()) {
      Some(u) => u,
      None => bail!("No database url was provided in the environment or via a command arg.")
    };
    let driver = get_driver(&url).chain_err(|| "Failed to get DB connection")?;

    let start = Instant::now();

    match matches.subcommand_name() {
        Some("status") => cmd::status(driver, &migration_files)?,
        Some("up") => cmd::up(driver, &migration_files)?,
        Some("down") => cmd::down(driver, &migration_files)?,
        Some("redo") => cmd::redo(driver, &migration_files)?,
        Some("revert") => cmd::revert(driver, &migration_files)?,
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

    let duration = start.elapsed();
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    // Spacing
    println!("");
    if minutes == 0 && seconds == 0 {
        println!("Operation took less than 1 second");
    } else {
        println!("Operation took {} minutes and {} seconds", minutes, seconds);
    }

    Ok(())
}
