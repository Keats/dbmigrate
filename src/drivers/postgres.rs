use postgres::{self, SslMode};

use super::Driver;
use errors::{MigrateResult};


#[derive(Debug)]
struct Postgres {
    conn: postgres::Connection
}

impl Driver for Postgres {
    type DriverStruct = Postgres;

    fn new(url: &str) -> MigrateResult<Self::DriverStruct> {
        let conn = try!(postgres::Connection::connect(url, &SslMode::None));

        Ok(Postgres{ conn: conn })
    }

    fn create_version_table(&self) {
        self.conn.batch_execute("
            CREATE TABLE IF NOT EXISTS migrations_table(id INTEGER, current INTEGER);
            INSERT INTO migrations_table (id, current)
            SELECT 1, 0
            WHERE NOT EXISTS(SELECT * FROM migrations_table WHERE id = 1);
        ").unwrap();
    }

    fn get_current_version(&self) -> i32 {
        let stmt = self.conn.prepare("
            SELECT current FROM migrations_table WHERE id = 1
        ").unwrap();
        let results = stmt.query(&[]).unwrap();

        let current_number: i32 = results.get(0).get("current");

        current_number
    }
}

#[cfg(test)]
mod tests {
    use super::{Postgres};
    use drivers::Driver;

    #[test]
    fn test_can_connect_to_test_db() {
        let pg = Postgres::new("postgres://pg@localhost:5432/migrate");
        assert_eq!(pg.is_ok(), true);
    }

    #[test]
    fn test_errors_on_unreachable_db() {
        let pg = Postgres::new("postgres://pg@localhost:5435/migrate");
        assert_eq!(pg.is_ok(), false);
    }

    #[test]
    fn test_should_create_the_table_only_once() {
        let pg = Postgres::new("postgres://pg@localhost:5432/migrate").unwrap();
        pg.create_version_table();
        assert_eq!(pg.get_current_version(), 0);
        pg.create_version_table();
        assert_eq!(pg.get_current_version(), 0);
    }
}
