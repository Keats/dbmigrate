# dbmigrate

[![Build Status](https://travis-ci.org/Keats/dbmigrate.svg)](https://travis-ci.org/Keats/dbmigrate)


A tool to create and manage SQL migrations.


## Databases supported

- Postgres
- MySQL
- Sqlite

## Usage

### Using CLI
Every call to dbmigrate requires 2 arguments: database url and migrations folder.
Those can be set through environment variables: `DBMIGRATE_URL` and `DBMIGRATE_PATH`, using a .env file, or as args to a call. Argument will override an environment variable.

```bash
# create a migration file
dbmigrate --url postgres://.. --path ./migrations create my_name
# apply all non applied migrations
dbmigrate --url postgres://.. --path ./migrations up
# un-apply all migrations
dbmigrate --url postgres://.. --path ./migrations down
# redo the last migration
dbmigrate --url postgres://.. --path ./migrations redo
# revert the last migration
dbmigrate --url postgres://.. --path ./migrations revert
# see list of migrations and which one is currently applied
dbmigrate --url postgres://.. --path ./migrations status
```

The format of the migration files is the following:
```bash
0001.initial_db.up.sql
0001.initial_db.down.sql
```

You can also pass a string to `create` and dbmigrate will slugify it for you:

```bash
dbmigrate --url postgres://.. --path ./migrations create "change currency table"

# gives the following files
0001.change_currency_table.up.sql
0001.change_currency_table.down.sql
```

`.` (dot) is not allowed in a migration name as it is the filename separator character.

### Using the library
Migrations can also be done programmatically and is how the CLI tool is built.

You will need to add the `dbmigrate-lib` dependency to your `Cargo.toml` file. The best example to see how how to make it work
is to look at the `dbmigrate` directory, which uses it to implement the CLI tool.


## Test locally
Build the project first with `cargo build`.
Assuming you use the docker images in the Makefile for pg and mysql:

Postgres:

```bash
./target/release/dbmigrate --url=postgres://pg@localhost:5777/migrate --path=/my/full/path/migrations status
```

MySQL:
```bash
./target/release/dbmigrate --url=mysql://mg:pass@localhost:5789/migrate --path=/my/full/path/migrations status
```

For Sqlite I have a Sqlite db named `dbmigrate.db` in the repo (gitignored):
```bash
./target/release/dbmigrate --path=/home/vincent/Code/dbmigrate/examples/migrations --url=sqlite:///dbmigrate.db status 
```


## Changelog

- 0.3.0: Add dotenv support + refactor error handling + update dependencies
- 0.2.7: Update docs to mention sqlite support
- 0.2.6: Support Sqlite
- 0.2.5: Update dependencies
- 0.2.4: Do not require DB connection to create migration file and update dependencies
- 0.2.3: don't panic on invalid files in migration folder & ssl support for postgres
- 0.2.2: slugify migration names and check if they are ok

## Acknowledgments
This is heavily inspired by [https://github.com/mattes/migrate](https://github.com/mattes/migrate).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
