use std::default::Default;

use mysql_client::conn::MyOpts;
use mysql_client::value::from_row;
use mysql_client::conn::pool::MyPool;
use url::Url;

use super::Driver;
use errors::{MigrateResult};


#[derive(Debug)]
pub struct Mysql {
    pool: MyPool
}

impl Mysql {
    pub fn new(url: Url) -> MigrateResult<Mysql> {
        let opts = MyOpts {
            tcp_addr: url.domain().map(|d| d.to_owned()),
            tcp_port: url.port_or_default().unwrap(),
            user: url.username().map(|d| d.to_owned()),
            pass: url.password().map(|d| d.to_owned()),
            db_name: url.path().map(|d| d[0].clone()),
            ..Default::default()
        };

        let pool = try!(MyPool::new(opts));
        let mysql = Mysql{ pool: pool };
        mysql.ensure_migration_table_exists();

        Ok(mysql)
    }
}


impl Driver for Mysql {
    fn ensure_migration_table_exists(&self) {
        self.pool.prep_exec("
            CREATE TABLE IF NOT EXISTS migrations_table(id INTEGER, current INTEGER);
            INSERT INTO migrations_table (id, current)
            SELECT 1, 0 FROM DUAL
            WHERE NOT EXISTS(SELECT * FROM migrations_table WHERE id = 1);
        ", ()).unwrap();
    }

    fn remove_migration_table(&self) {
        self.pool.prep_exec("DROP TABLE migrations_table;", ()).unwrap();
    }

    fn get_current_number(&self) -> i32 {
        let mut result = self.pool.prep_exec("
            SELECT current FROM migrations_table WHERE id = 1;
        ", ()).unwrap();
        // That is quite ugly
        let row = result.next().unwrap();
        from_row::<i32>(row.unwrap())
    }

    fn set_current_number(&self, number: i32) {
        self.pool.prep_exec(
            "UPDATE migrations_table SET current = $1 WHERE id = 1;",
            (&number,)
        ).unwrap();
    }

    fn migrate(&self, migration: String, number: i32) -> MigrateResult<()> {
        try!(self.pool.prep_exec(&migration, ()));
        self.set_current_number(number);

        Ok(())
    }
}
