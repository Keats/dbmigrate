///! Driver interface and implementations
use errors::{MigrateResult};


pub trait Driver {
    type DriverStruct;
    // TODO: the whole thing with the custom type looks a bit ugly
    fn new(url: &str) -> MigrateResult<Self::DriverStruct>;
    fn create_version_table(&self);
    fn get_current_version(&self) -> i32;
    // fn migrate() -> MigrateResult<()>;
}


mod postgres;
