use url::Url;

use sqlcipher_client::Connection;

use super::Driver;
use errors::{Result, ResultExt};

const SQLCIPHER_DEFAULT_CIPHER: &'static str = "AES-256-CBC";

#[derive(Debug)]
pub struct Sqlcipher {
    conn: Connection,
}

impl Sqlcipher {
    pub fn new(url: &str) -> Result<Sqlcipher> {
        // Expect a URL in the form:
        // sqlcipher://user:password@host/path/to/file.db
        //
        // The user and host components are not used.
        // The password is used with the PRAGMA KEY statement.
        let parsedurl = Url::parse(url)
            .chain_err(|| format!("Invalid SQLCipher URL: {}", url))?;
        let path = parsedurl.path();
        let conn = Connection::open(path)?;
        let sqlcipher = Sqlcipher { conn: conn };
        match parsedurl.password() {
            Some(password) => {
                println!("Password: {:?}", password);
                sqlcipher.unlock(password)?;
                sqlcipher.verify_cipher()?;
                sqlcipher.ensure_migration_table_exists();
            }
            None => bail!("No password found in URL"),
        }
        Ok(sqlcipher)
    }

    fn unlock(&self, password: &str) -> Result<()> {
        let pragmakey = format!("PRAGMA KEY = '{}'", password);
        match self.conn.execute(&pragmakey, &[]) {
            Ok(rows) => assert!(rows == 0),
            Err(err_) => bail!("Failed to set database key: {}", err_),
        };
        Ok(())
    }

    fn verify_cipher(&self) -> Result<()> {
        // This function exists to verify that the database is encrypted.  Any
        // failure in this code should not result in private data being
        // exposed.
        //
        // By default, PRAGMA CIPHER simply returns "AES-256-CBC".
        let mut statement = self.conn.prepare("PRAGMA CIPHER")?;
        match statement.query(&[]) {
            Ok(mut rows) => {
                let mut rowcount: usize = 0;
                match rows.next() {
                    Some(Ok(row)) => {
                        rowcount += 1;
                        let result: String = row.get_checked(0)?;
                        assert_eq!(SQLCIPHER_DEFAULT_CIPHER, result);
                    }
                    Some(Err(err)) => {
                        bail!("Failed to retrieve the cipher: {}", err)
                    }
                    None => bail!("Cipher query returned no rows."),
                };
                assert!(rowcount == 1);
            }
            Err(err) => bail!("Failed to verify the cipher: {}", err),
        };
        Ok(())
    }
}

impl Driver for Sqlcipher {
    fn ensure_migration_table_exists(&self) {
        self.conn
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS
                    __dbmigrate_table(id INTEGER, current INTEGER);
                INSERT INTO
                    __dbmigrate_table (id, current)
                SELECT
                    1, 0
                WHERE NOT EXISTS(
                    SELECT * FROM __dbmigrate_table WHERE id = 1
                );",
            )
            .unwrap();
    }

    fn remove_migration_table(&self) {
        self.conn
            .execute("DROP TABLE __dbmigrate_table;", &[])
            .unwrap();
    }

    fn get_current_number(&self) -> i32 {
        self.conn
            .query_row(
                "SELECT current FROM __dbmigrate_table WHERE id = 1;",
                &[],
                |row| row.get(0),
            )
            .unwrap()
    }

    fn set_current_number(&self, number: i32) {
        let mut stmt = self.conn
            .prepare("UPDATE __dbmigrate_table SET current = ? WHERE id = 1;")
            .unwrap();
        stmt.execute(&[&number]).unwrap();
    }

    fn migrate(&self, migration: String, number: i32) -> Result<()> {
        self.conn
            .execute_batch(&migration)
            .chain_err(|| "Migration failed")?;
        self.set_current_number(number);

        Ok(())
    }
}
