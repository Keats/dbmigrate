use postgres::{self, SslMode};

use super::Driver;
use errors::{MigrateResult};


#[derive(Debug)]
pub struct Postgres {
    conn: postgres::Connection
}

impl Postgres {
    pub fn new(url: &str) -> MigrateResult<Postgres> {
        let conn = try!(postgres::Connection::connect(url, &SslMode::None));
        let pg = Postgres{ conn: conn };
        pg.ensure_migration_table_exists();

        Ok(pg)
    }
}

impl Driver for Postgres {
    fn ensure_migration_table_exists(&self) {
        self.conn.batch_execute("
            CREATE TABLE IF NOT EXISTS migrations_table(id INTEGER, current INTEGER);
            INSERT INTO migrations_table (id, current)
            SELECT 1, 0
            WHERE NOT EXISTS(SELECT * FROM migrations_table WHERE id = 1);
        ").unwrap();
    }

    fn remove_migration_table(&self) {
        self.conn.execute("DROP TABLE migrations_table;", &[]).unwrap();
    }

    fn get_current_number(&self) -> i32 {
        let stmt = self.conn.prepare("
            SELECT current FROM migrations_table WHERE id = 1
        ").unwrap();
        let results = stmt.query(&[]).unwrap();

        results.get(0).get("current")
    }

    fn set_current_number(&self, number: i32) {
        let stmt = self.conn.prepare(
            "UPDATE migrations_table SET current = $1 WHERE id = 1;"
        ).unwrap();
        stmt.execute(&[&number]).unwrap();
    }

    fn migrate(&self, migration: String, number: i32) -> MigrateResult<()> {
        try!(self.conn.batch_execute(&migration));
        self.set_current_number(number);

        Ok(())
    }
}
