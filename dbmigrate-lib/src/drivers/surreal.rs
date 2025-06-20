use surreal_client::Surreal;
use surreal_client::engine::remote::ws::{Client, Ws};
use tokio::runtime::Runtime;
use url::Url;

use super::Driver;
use crate::errors::{Result, ResultExt};

/// The SurrealDB driver
#[derive(Debug)]
pub struct Surrealdb {
    client: Surreal<Client>,
    runtime: Runtime,
}

impl Surrealdb {
    /// Create SurrealDB driver
    pub fn new(url: &str) -> Result<Surrealdb> {
        let runtime =
            Runtime::new().chain_err(|| format!("Tokio runtime failed to start: {}", url))?;

        let parsed_url = Url::parse(url).chain_err(|| format!("Invalid SurrealDB URL: {}", url))?;

        let host = parsed_url
            .host_str()
            .ok_or_else(|| format!("No host in URL: {}", url))?;

        let port = parsed_url
            .port()
            .ok_or_else(|| format!("No port in URL: {}", url))?;

        let endpoint = format!("{}:{}", host, port);

        let username = parsed_url.username();
        let password = parsed_url
            .password()
            .ok_or_else(|| format!("No password in URL: {}", url))?;

        let path_segments: Vec<&str> = parsed_url
            .path()
            .trim_start_matches('/')
            .split('/')
            .collect();
        if path_segments.len() < 2 {
            return Err(format!(
                "SurrealDB URL must include namespace and database in URL: {}",
                url
            )
            .into());
        }
        let namespace = path_segments[0];
        let database = path_segments[1];

        let client = runtime
            .block_on(async {
                let client = Surreal::new::<Ws>(&endpoint)
                    .await
                    .chain_err(|| format!("Failed to connect to SurrealDB at {}", endpoint))?;

                client
                    .signin(surrealdb::opt::auth::Database {
                        username,
                        password,
                        namespace,
                        database,
                    })
                    .await
                    .chain_err(|| "Failed to authenticate in SurrealDB")?;

                Ok::<_, crate::errors::Error>(client)
            })
            .chain_err(|| "Failed to create SurrealDB client")?;

        let mut surrealdb = Surrealdb { client, runtime };

        surrealdb.ensure_migration_table_exists();

        Ok(surrealdb)
    }
}

impl Driver for Surrealdb {
    fn ensure_migration_table_exists(&mut self) {
        self.runtime.block_on(async {
            let query = r#"
                DEFINE TABLE IF NOT EXISTS __dbmigrate_table SCHEMAFULL;
                DEFINE FIELD IF NOT EXISTS id ON TABLE __dbmigrate_table TYPE int;
                DEFINE FIELD IF NOT EXISTS current ON TABLE __dbmigrate_table TYPE int;

                LET $initialMigration = (SELECT * FROM ONLY __dbmigrate_table:1);
                IF !$initialMigration {
                    CREATE __dbmigrate_table:1 SET current = 0;
                }
            "#;

            self.client.query(query).await.unwrap();
        });
    }

    fn remove_migration_table(&mut self) {
        self.runtime.block_on(async {
            let query = "REMOVE TABLE __dbmigrate_table;";
            self.client.query(query).await.unwrap();
        });
    }

    fn get_current_number(&mut self) -> u32 {
        self.runtime.block_on(async {
            let query = r#"
                let $record = SELECT current FROM ONLY __dbmigrate_table:1;
                RETURN $record.current;
            "#;
            let mut result = self.client.query(query).await.unwrap();

            let current = result.take::<Option<u32>>(1).unwrap();

            current.unwrap_or(0)
        })
    }

    fn set_current_number(&mut self, number: u32) {
        self.runtime.block_on(async {
            let query = "UPDATE __dbmigrate_table:1 SET current = $number;";
            self.client
                .query(query)
                .bind(("number", number))
                .await
                .unwrap();
        });
    }

    fn migrate(&mut self, migration: String, number: u32) -> Result<()> {
        self.runtime.block_on(async {
            self.client
                .query(&migration)
                .await
                .chain_err(|| "Migration failed")
        })?;

        self.set_current_number(number);

        Ok(())
    }
}
