///! Driver interface and implementations
use errors::{MigrateResult};
use files::{MigrationFile};


pub trait Driver {
    type DriverStruct;
    // TODO: the whole thing with the custom type looks a bit ugly
    fn new(url: &str) -> MigrateResult<Self::DriverStruct>;
    fn ensure_migration_table_exists(&self);
    fn remove_migration_table(&self);
    fn get_current_number(&self) -> i32;
    fn set_current_number(&self, number: i32);
    fn migrate(&self, migration: MigrationFile);
}


mod postgres;
