use mysql_client::{from_row, Pool};

use super::Driver;
use errors::{Result, ResultExt};


/// The MySQL driver
#[derive(Debug)]
pub struct Mysql {
    pool: Pool
}

impl Mysql {
    /// Create MySQL driver
    pub fn new(url: &str) -> Result<Mysql> {
        let pool = Pool::new(url)?;
        let mut mysql = Mysql { pool: pool };
        mysql.ensure_migration_table_exists();

        Ok(mysql)
    }
}


impl Driver for Mysql {
    fn ensure_migration_table_exists(&mut self) {
        let mut conn = self.pool.get_conn().unwrap();
        conn.query("
            CREATE TABLE IF NOT EXISTS __dbmigrate_table(id INTEGER, current INTEGER);
            INSERT INTO __dbmigrate_table (id, current)
            SELECT 1, 0 FROM DUAL
            WHERE NOT EXISTS(SELECT * FROM __dbmigrate_table WHERE id = 1);
        ").unwrap();
    }

    fn remove_migration_table(&mut self) {
        self.pool.prep_exec("DROP TABLE __dbmigrate_table;", ()).unwrap();
    }

    fn get_current_number(&mut self) -> i32 {
        let mut result = self.pool.prep_exec("
            SELECT current FROM __dbmigrate_table WHERE id = 1;
        ", ()).unwrap();
        // That is quite ugly
        let row = result.next().unwrap();
        from_row::<i32>(row.unwrap())
    }

    fn set_current_number(&mut self, number: i32) {
        self.pool.prep_exec(
            "UPDATE __dbmigrate_table SET current = ? WHERE id = 1;",
            (&number, )
        ).unwrap();
    }

    fn migrate(&mut self, migration: String, number: i32) -> Result<()> {
        let mut conn = self.pool.get_conn()?;
        conn.query(migration).chain_err(|| "Migration failed")?;
        self.set_current_number(number);

        Ok(())
    }
}
