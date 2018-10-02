use url::Url;

use sqlcipher_client::Connection;

use super::Driver;
use errors::{Result, ResultExt};

#[derive(Debug)]
pub struct Sqlcipher {
    conn: Connection,
}

impl Sqlcipher {
    /// Open an SQLCipher database
    ///
    /// This method expects a database URL in the form
    /// sqlcipher://$USER:$PASSWORD@$HOST/$DATABASEPATH
    ///
    /// The USER and HOST values will not be used.
    /// The PASSWORD will be used with the PRAGMA KEY statement.
    pub fn new(url: &str) -> Result<Sqlcipher> {
        let parsed_url = Url::parse(url)
            .chain_err(|| format!("Invalid SQLCipher URL: {}", url))?;
        let path = parsed_url.path();
        let conn = Connection::open(path)
            .chain_err(|| format!("Unable to open database at {}", path))?;
        let sqlcipher = Sqlcipher { conn: conn };
        match parsed_url.password() {
            Some(password) => {
                sqlcipher.unlock(password)?;
                sqlcipher.verify_sqlcipher()?;
                sqlcipher.verify_valid_key()?;
                sqlcipher.ensure_migration_table_exists();
            }
            None => bail!("No password found in URL"),
        }
        Ok(sqlcipher)
    }

    /// Unlock the SQLCipher database with the provided password
    ///
    /// This function uses the PRAGMA KEY
    fn unlock(&self, password: &str) -> Result<()> {
        let mut statement = self.conn.prepare(&format!("PRAGMA KEY = '{}'", password))?;
        let results = statement.query_map(&[], |row| row.get::<usize, String>(0))?;
        assert!(results.count() == 0);
        Ok(())
    }

    /// Verify that the underlying library acts like SQLCipher
    ///
    /// This should prevent the user from continuing if the library is
    /// accidentally linked against SQLite instead of SQLCipher.
    fn verify_sqlcipher(&self) -> Result<()> {
        let mut statement = self.conn.prepare(
            "PRAGMA cipher_version")?;
        match statement.query(&[]) {
            Ok(mut rows) => {
                let mut row_count: usize = 0;
                match rows.next() {
                    Some(Ok(row)) => {
                        row_count += 1;
                        let result: String = row.get_checked(0)?;
                        assert!(result.len() > 0);
                    },
                    Some(Err(err)) => {
                        bail!("PRAGMA cipher_version returned an error: {}", err)
                    },
                    // Verified.
                    None => bail!("This tool is linked against SQLite instead of SQLCipher")
                };
                assert!(row_count == 1);
            },
            Err(err) => bail!(
                "Failed to successfully complete PRAGMA cipher_version query: {}", err),
        };
        Ok(())
    }

    /// Verify that the provided password is valid
    ///
    /// This function is present to prevent sensitive data being added to a
    /// database which is not encrypted.
    fn verify_valid_key(&self) -> Result<()> {
        match self.conn.prepare("SELECT count(*) FROM sqlite_master") {
            Ok(mut statement) => {
                match statement.query(&[]) {
                    Ok(mut rows) => {
                        let mut row_count: usize = 0;
                        match rows.next() {
                            Some(Ok(row)) => {
                                row_count += 1;
                                let result: i64 = row.get_checked(0)?;
                                assert!(result >= 0);
                            }
                            Some(Err(err)) => {
                                bail!("sqlite_master query returned an error: {}", err)
                            }
                            None => bail!("sqlite_master query returned nothing"),
                        };
                        assert!(row_count == 1);
                    }
                    Err(err) => bail!("Failed to query the sqlite_master table: {}", err),
                };
            },
            Err(err) => {
                bail!("This database may not be encrypted or may require a different password: {}", err);
            }
        }
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
