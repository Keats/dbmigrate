use std::str::FromStr;

use native_tls::TlsConnector;
use postgres_client::{Client, Config};
use postgres_native_tls::{MakeTlsConnector};

use super::Driver;
use errors::{Result, ResultExt};

/// The PostgreSQL driver
//#[derive(Debug)]
pub struct Postgres {
    client: Client
}

impl Postgres {
    /// Create PostgreSQL driver
    pub fn new(url: &str) -> Result<Postgres> {
        let config = Config::from_str(url)?;
        let connector = TlsConnector::new().unwrap();
        let connector = MakeTlsConnector::new(connector);
        let client = config.connect(connector)?;
        Postgres::from_client(client)
    }
    /// Create PostgreSQL driver using an existing client
    pub fn from_client(client: Client) -> Result<Postgres> {
        let mut pg = Postgres { client: client };
        pg.ensure_migration_table_exists();
        Ok(pg)
    }
}

impl Driver for Postgres {
    fn ensure_migration_table_exists(&mut self) {
        self.client.simple_query("
            CREATE TABLE IF NOT EXISTS __dbmigrate_table(id INTEGER, current INTEGER);
            INSERT INTO __dbmigrate_table (id, current)
            SELECT 1, 0
            WHERE NOT EXISTS(SELECT * FROM __dbmigrate_table WHERE id = 1);
        ").unwrap();
    }

    fn remove_migration_table(&mut self) {
        self.client.execute("DROP TABLE __dbmigrate_table;", &[]).unwrap();
    }

    fn get_current_number(&mut self) -> i32 {
        let stmt = self.client.prepare("
            SELECT current FROM __dbmigrate_table WHERE id = 1;
        ").unwrap();
        let results = self.client.query(&stmt, &[]).unwrap();
        results.first().unwrap().get("current")
    }

    fn set_current_number(&mut self, number: i32) {
        let stmt = self.client.prepare(
            "UPDATE __dbmigrate_table SET current = $1 WHERE id = 1;"
        ).unwrap();
        self.client.execute(&stmt, &[&number]).unwrap();
    }

    fn migrate(&mut self, migration: String, number: i32) -> Result<()> {
        self.client.simple_query(&migration).chain_err(|| "Migration failed")?;
        self.set_current_number(number);
        Ok(())
    }
}
