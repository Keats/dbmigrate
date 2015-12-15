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

        let current_number: i32 = results.get(0).get("current");

        current_number
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

#[cfg(test)]
mod tests {
    use super::{Postgres};
    use drivers::Driver;
    use files::{MigrationFile, Direction};
    use std::env;

    #[test]
    fn test_can_connect_to_test_db() {
        let pg = Postgres::new(&env::var("DBMIGRATE_URL").unwrap());
        assert_eq!(pg.is_ok(), true);
    }

    #[test]
    fn test_errors_on_unreachable_db() {
        let pg = Postgres::new("postgres://pg@localhost:5435/migrate");
        assert_eq!(pg.is_ok(), false);
    }

    #[test]
    fn test_should_update_current_migration_after_migrating_up() {
        let mig = MigrationFile{
            content: Some("CREATE TABLE IF NOT EXISTS blob();".to_owned()),
            number: 42,
            direction: Direction::Up,
            filename: "".to_owned(),
            name: "".to_owned(),
        };
        let pg = Postgres::new(&env::var("DBMIGRATE_URL").unwrap()).unwrap();
        pg.migrate(mig.content.unwrap(), mig.number);
        assert_eq!(pg.get_current_number(), 42);
        pg.set_current_number(0);
    }

    #[test]
    fn test_should_update_current_migration_after_migrating_down() {
        let mig = MigrationFile{
            content: Some("DROP TABLE IF EXISTS blob;".to_owned()),
            number: 42,
            direction: Direction::Down,
            filename: "".to_owned(),
            name: "".to_owned(),
        };
        let pg = Postgres::new(&env::var("DBMIGRATE_URL").unwrap()).unwrap();
        pg.migrate(mig.content.unwrap(), mig.number - 1);
        assert_eq!(pg.get_current_number(), 41);
        pg.set_current_number(0);
    }
}
