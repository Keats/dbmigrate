use sqlite_client::Connection;

use super::Driver;
use errors::{Result, ResultExt};


#[derive(Debug)]
pub struct Sqlite {
    conn: Connection
}

impl Sqlite {
    pub fn new(url: &str) -> Result<Sqlite> {
        // the replace is probably wrong
        let conn = Connection::open(url.replace("sqlite:/", ""))?;
        let sqlite = Sqlite { conn: conn };
        sqlite.ensure_migration_table_exists();
        Ok(sqlite)
    }
}

impl Driver for Sqlite {
    fn ensure_migration_table_exists(&self) {
        self.conn.execute_batch("
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
        self.conn.query_row("
            SELECT current FROM __dbmigrate_table WHERE id = 1;
        ", &[], |row| { row.get(0)}).unwrap()
    }

    fn set_current_number(&self, number: i32) {
        let mut stmt = self.conn.prepare(
            "UPDATE __dbmigrate_table SET current = ? WHERE id = 1;"
        ).unwrap();
        stmt.execute(&[&number]).unwrap();
    }

    fn migrate(&self, migration: String, number: i32) -> Result<()> {
        self.conn.execute_batch(&migration).chain_err(|| "Migration failed")?;
        self.set_current_number(number);

        Ok(())
    }
}
