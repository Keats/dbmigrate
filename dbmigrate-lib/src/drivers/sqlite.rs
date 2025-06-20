use sqlite_client::Connection;

use super::Driver;
use crate::errors::{Result, ResultExt};

/// The SQLite driver
#[derive(Debug)]
pub struct Sqlite {
    conn: Connection,
}

impl Sqlite {
    /// Create SQLite driver
    pub fn new(url: &str) -> Result<Sqlite> {
        // the replace is probably wrong
        let conn = Connection::open(url.replace("sqlite:/", ""))?;
        let mut sqlite = Sqlite { conn: conn };
        sqlite.ensure_migration_table_exists();
        Ok(sqlite)
    }
}

impl Driver for Sqlite {
    fn ensure_migration_table_exists(&mut self) {
        self.conn
            .execute_batch(
                "
            CREATE TABLE IF NOT EXISTS __dbmigrate_table(id INTEGER, current INTEGER);
            INSERT INTO __dbmigrate_table (id, current)
            SELECT 1, 0
            WHERE NOT EXISTS(SELECT * FROM __dbmigrate_table WHERE id = 1);
        ",
            )
            .unwrap();
    }

    fn remove_migration_table(&mut self) {
        self.conn
            .execute("DROP TABLE __dbmigrate_table;", &[])
            .unwrap();
    }

    fn get_current_number(&mut self) -> u32 {
        self.conn
            .query_row(
                "
            SELECT current FROM __dbmigrate_table WHERE id = 1;
        ",
                &[],
                |row| row.get(0),
            )
            .unwrap()
    }

    fn set_current_number(&mut self, number: u32) {
        let mut stmt = self
            .conn
            .prepare("UPDATE __dbmigrate_table SET current = ? WHERE id = 1;")
            .unwrap();
        stmt.execute(&[&number]).unwrap();
    }

    fn migrate(&mut self, migration: String, number: u32) -> Result<()> {
        self.conn
            .execute_batch(&migration)
            .chain_err(|| "Migration failed")?;
        self.set_current_number(number);

        Ok(())
    }
}
