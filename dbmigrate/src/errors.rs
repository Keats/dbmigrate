use dbmigrate_lib::errors;

error_chain! {
    links {
        DbMigrateLib(errors::Error, errors::ErrorKind);
    }
}
