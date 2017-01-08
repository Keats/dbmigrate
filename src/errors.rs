use postgres_client;
use mysql_client;
use sqlite_client;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Postgres(postgres_client::error::ConnectError);
        MySQL(mysql_client::Error);
        Sqlite(sqlite_client::Error);
    }
}
