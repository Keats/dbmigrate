use postgres_client;
use mysql_client;
use sqlite_client;

error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "Failed to created/read migration files"];
        Postgres(postgres_client::error::ConnectError) #[doc = "Couldn't get connection to pg database"];
        MySQL(mysql_client::Error) #[doc = "Any MySQL error"];
        Sqlite(sqlite_client::Error) #[doc = "Any Sqlite error"];
    }
}
