#[cfg(feature = "mysql_support")]
use mysql_client;
#[cfg(feature = "postgres_support")]
use postgres_client;
#[cfg(feature = "sqlite_support")]
use sqlite_client;
#[cfg(feature = "surreal_support")]
use surreal_client;

error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "Failed to created/read migration files"];
        Postgres(postgres_client::error::Error) #[doc = "Couldn't get connection to pg database"] #[cfg(feature = "postgres_support")];
        MySQL(mysql_client::Error) #[doc = "Any MySQL error"] #[cfg(feature = "mysql_support")];
        Sqlite(sqlite_client::Error) #[doc = "Any Sqlite error"] #[cfg(feature = "sqlite_support")];
        Surreal(surreal_client::Error) #[doc = "Any SurrealDB error"] #[cfg(feature = "surreal_support")];
    }
}
