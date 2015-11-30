///! Driver interface and implementations
use errors::{MigrateResult};
use files::{MigrationFile};
use drivers::pg::Postgres;


// That approach doesn't work as it doesn't seem possible to return
// a generic implementing a specific trait
pub trait Driver {
    fn ensure_migration_table_exists(&self);
    fn remove_migration_table(&self);
    fn get_current_number(&self) -> i32;
    fn set_current_number(&self, number: i32);
    fn migrate(&self, migration: MigrationFile);
}

// so we only care about pg
pub mod pg;
