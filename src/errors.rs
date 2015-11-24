use std::io;

use postgres;


#[derive(Debug)]
pub enum LibError {
    /// Found a file not using the right format for name
    InvalidFilename,
    /// Migration number jumped, ie going from 1 to 3
    MigrationSkipped,
    /// Either the up or down file for a migration was missing
    MissingFile,
    /// IO error
    Io(io::Error),
    /// Couldn't connect to the PG database
    PostgresConnection(postgres::error::ConnectError),
    /// An error occured when running a SQL query in PG
    PostgresError(postgres::error::Error)
}

macro_rules! impl_from_error {
    ($f: ty, $e: expr) => {
        impl From<$f> for LibError {
            fn from(f: $f) -> LibError { $e(f) }
        }
    }
}

impl_from_error!(io::Error, LibError::Io);
impl_from_error!(postgres::error::ConnectError, LibError::PostgresConnection);
impl_from_error!(postgres::error::Error, LibError::PostgresError);

/// Library generic result type.
pub type MigrateResult<T> = Result<T, LibError>;
