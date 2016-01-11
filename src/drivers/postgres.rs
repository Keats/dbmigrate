use postgres_client::{self, SslMode};

use super::Driver;
use errors::{MigrateResult};


#[derive(Debug)]
pub struct Postgres {
    conn: postgres_client::Connection
}

impl Postgres {
    pub fn new(url: &str) -> MigrateResult<Postgres> {
        let conn = try!(postgres_client::Connection::connect(url, SslMode::None));
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
