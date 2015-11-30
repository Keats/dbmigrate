use std::io;
use std::fmt;
use std::error::Error;
use std::process;
use postgres;


/// Library generic result type.
pub type MigrateResult<T> = Result<T, MigrateError>;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MigrateErrorType {
    /// No migration directory was provided
    NoMigrationPath,
    /// No database url was provided
    NoDatabaseUrl,
    /// Found a file not using the right format for name
    InvalidFilename,
    /// Migration number jumped, ie going from 1 to 3
    MigrationSkipped,
    /// Either the up or down file for a migration was missing
    MissingFile,
    /// IO error
    Io,
    /// Couldn't connect to the PG database
    PostgresConnection,
    /// An error occured when running a SQL query in PG
    PostgresError
}

/// Our actual error
#[derive(Debug)]
pub struct MigrateError {
    /// The error message
    pub error: String,
    /// The error type
    pub error_type: MigrateErrorType
}

macro_rules! wlnerr(
    ($($arg:tt)*) => ({
        use std::io::{Write, stderr};
        writeln!(&mut stderr(), $($arg)*).ok();
    })
);

impl MigrateError {
    pub fn exit(&self) -> ! {
        wlnerr!("{}", self.error);
        process::exit(1);
    }
}

impl Error for MigrateError {
    fn description(&self) -> &str {
        &*self.error
    }

    fn cause(&self) -> Option<&Error> {
        match self.error_type {
            _ => None,
        }
    }
}

impl fmt::Display for MigrateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.error)
    }
}


macro_rules! impl_from_error {
    ($f: ty, $e: expr) => {
        impl From<$f> for MigrateError {
            fn from(e: $f) -> Self {
                MigrateError {
                    error: format!("{}", e.description()),
                    error_type: $e
                }
            }
        }
    }
}

impl_from_error!(io::Error, MigrateErrorType::Io);
impl_from_error!(postgres::error::ConnectError, MigrateErrorType::PostgresConnection);
impl_from_error!(postgres::error::Error, MigrateErrorType::PostgresError);


pub fn invalid_filename(filename: &str) -> MigrateError {
    MigrateError {
        error: format!("Found a file with an invalid name: {}", filename),
        error_type: MigrateErrorType::InvalidFilename
    }
}

pub fn migration_skipped(number: i32) -> MigrateError {
    MigrateError {
        error: format!("Files for migration {} are missing.", number),
        error_type: MigrateErrorType::MigrationSkipped
    }
}

pub fn missing_file(number: i32) -> MigrateError {
    MigrateError {
        error: format!("Migration {} is missing its up or down file", number),
        error_type: MigrateErrorType::MissingFile
    }
}

pub fn no_migration_path() -> MigrateError {
    MigrateError {
        error: format!("No migration path was provided in the environment or via a command arg."),
        error_type: MigrateErrorType::NoMigrationPath
    }
}

pub fn no_database_url() -> MigrateError {
    MigrateError {
        error: format!("No database url was provided in the environment or via a command arg."),
        error_type: MigrateErrorType::NoDatabaseUrl
    }
}
