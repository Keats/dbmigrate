use std::io::{self, Write};
use std::fmt;
use std::error::Error;
use std::process;

use postgres_client;
use mysql_client;
use openssl;
use url;

use print;

/// Library generic result type.
pub type MigrateResult<T> = Result<T, MigrateError>;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MigrateErrorType {
    /// Invalid URL
    InvalidUrl,
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
    PostgresError,
    /// Couldn't connect to the mysql database or migration failed
    MysqlError,
    /// Couldn't create OpenSSL context
    OpenSslError,
}

/// Our actual error
#[derive(Debug)]
pub struct MigrateError {
    /// The error message
    pub error: String,
    /// The error type
    pub error_type: MigrateErrorType
}

impl MigrateError {
    pub fn exit(&self) -> ! {
        print::error(&self.error.clone());
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
                    error: e.description().to_owned(),
                    error_type: $e
                }
            }
        }
    }
}

impl_from_error!(io::Error, MigrateErrorType::Io);
impl_from_error!(openssl::ssl::error::SslError, MigrateErrorType::OpenSslError);
impl_from_error!(url::ParseError, MigrateErrorType::InvalidUrl);

impl From<postgres_client::error::Error> for MigrateError {
    fn from(e: postgres_client::error::Error) -> Self {
        MigrateError {
            error: format!("{}", e),
            error_type: MigrateErrorType::PostgresError
        }
    }
}

impl From<postgres_client::error::ConnectError> for MigrateError {
    fn from(e: postgres_client::error::ConnectError) -> Self {
        MigrateError {
            error: format!("Postgres connection error.\n{}", e),
            error_type: MigrateErrorType::PostgresConnection
        }
    }
}

impl From<mysql_client::error::MyError> for MigrateError {
    fn from(e: mysql_client::error::MyError) -> Self {
        MigrateError {
            error: format!("MySQL error.\n{}", e),
            error_type: MigrateErrorType::MysqlError
        }
    }
}

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
        error: "No migration path was provided in the environment or via a command arg.".to_owned(),
        error_type: MigrateErrorType::NoMigrationPath
    }
}

pub fn no_database_url() -> MigrateError {
    MigrateError {
        error: "No database url was provided in the environment or via a command arg.".to_owned(),
        error_type: MigrateErrorType::NoDatabaseUrl
    }
}

pub fn invalid_url(url: &str) -> MigrateError {
    MigrateError {
        error: format!("URL provided is not supported (only postgres and mysql are supported): {}", url),
        error_type: MigrateErrorType::InvalidUrl
    }
}
