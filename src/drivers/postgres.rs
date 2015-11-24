use postgres::{self, SslMode};

use super::Driver;
use errors::{MigrateResult};
use files::{MigrationFile, Direction};


#[derive(Debug)]
struct Postgres {
    conn: postgres::Connection
}

impl Driver for Postgres {
    type DriverStruct = Postgres;

    fn new(url: &str) -> MigrateResult<Self::DriverStruct> {
        let conn = try!(postgres::Connection::connect(url, &SslMode::None));
        let pg = Postgres{ conn: conn };
        pg.ensure_migration_table_exists();

        Ok(pg)
    }

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

    fn migrate(&self, migration: MigrationFile) {
        self.conn.batch_execute(&migration.content.unwrap()).unwrap();
        // If we are migrating up, we can use the migration number otherwise
        // we do number - 1 to represent we are removing it
        let number = if migration.direction == Direction::Up {
            migration.number
        } else {
            migration.number - 1
        };
        self.set_current_number(number);
    }
}

#[cfg(test)]
mod tests {
    use super::{Postgres};
    use drivers::Driver;
    use files::{MigrationFile, Direction};

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
    fn test_should_update_current_migration_after_migrating_up() {
        let mig = MigrationFile{
            content: Some("CREATE TABLE IF NOT EXISTS blob();".to_owned()),
            number: 42,
            direction: Direction::Up,
            filename: "".to_owned(),
            name: "".to_owned(),
        };
        let pg = Postgres::new("postgres://pg@localhost:5432/migrate").unwrap();
        pg.migrate(mig);
        assert_eq!(pg.get_current_number(), 42);
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
        let pg = Postgres::new("postgres://pg@localhost:5432/migrate").unwrap();
        pg.migrate(mig);
        assert_eq!(pg.get_current_number(), 41);
    }
}
