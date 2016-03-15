use postgres_client::{self, Connection, SslMode};
use openssl::ssl::{SslContext, SslMethod};
use url::Url;

use super::Driver;
use errors::{MigrateError, MigrateResult};


#[derive(Debug)]
pub struct Postgres {
    conn: postgres_client::Connection
}

impl Postgres {
    pub fn new(url: &str) -> MigrateResult<Postgres> {
        let conn = try!(mk_connection(url));
        let pg = Postgres{ conn: conn };
        pg.ensure_migration_table_exists();
        Ok(pg)
    }
}

impl Driver for Postgres {
    fn ensure_migration_table_exists(&self) {
        self.conn.batch_execute("
            CREATE TABLE IF NOT EXISTS __dbmigrate_table(id INTEGER, current INTEGER);
            INSERT INTO __dbmigrate_table (id, current)
            SELECT 1, 0
            WHERE NOT EXISTS(SELECT * FROM __dbmigrate_table WHERE id = 1);
        ").unwrap();
    }

    fn remove_migration_table(&self) {
        self.conn.execute("DROP TABLE __dbmigrate_table;", &[]).unwrap();
    }

    fn get_current_number(&self) -> i32 {
        let stmt = self.conn.prepare("
            SELECT current FROM __dbmigrate_table WHERE id = 1;
        ").unwrap();
        let results = stmt.query(&[]).unwrap();

        results.get(0).get("current")
    }

    fn set_current_number(&self, number: i32) {
        let stmt = self.conn.prepare(
            "UPDATE __dbmigrate_table SET current = $1 WHERE id = 1;"
        ).unwrap();
        stmt.execute(&[&number]).unwrap();
    }

    fn migrate(&self, migration: String, number: i32) -> MigrateResult<()> {
        try!(self.conn.batch_execute(&migration));
        self.set_current_number(number);

        Ok(())
    }
}

fn mk_connection(url: &str) -> Result<Connection, MigrateError> {
    let ctx = try!(SslContext::new(SslMethod::Sslv23));
    let sslmode = try!(Url::parse(url))
        .query_pairs()
        .and_then(|pairs|
            pairs.into_iter().find(|&(ref k, _)| k == "sslmode"))
        .map(|(_k, v)| match v.as_str() {
            "allow" | "prefer" => SslMode::Prefer(&ctx),
            "require" => SslMode::Require(&ctx),
            // No support for certificate verification yet.
            "verify-ca" | "verify-full" => unimplemented!(),
            _ => SslMode::None
        })
        .unwrap_or(SslMode::None);
    postgres_client::Connection::connect(url, sslmode)
        .map_err(From::from)
}
