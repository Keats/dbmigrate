use std::io;

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
}

macro_rules! impl_from_error {
    ($f: ty, $e: expr) => {
        impl From<$f> for LibError {
            fn from(f: $f) -> LibError { $e(f) }
        }
    }
}

impl_from_error!(io::Error, LibError::Io);
