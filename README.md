# dbmigrate

[![Build Status](https://travis-ci.org/Keats/dbmigrate.svg)](https://travis-ci.org/Keats/dbmigrate)


A tool to create and manage SQL migrations.


## Databases supported

- Postgres
- MySQL

## Usage

Every call to dbmigrate requires 2 arguments: database url and migrations folder.
Those can be set through environment variables: `DBMIGRATE_URL` and `DBMIGRATE_PATH` or as args to a call. Argument will override an environment variable.

```bash
# create a migration file
dbmigrate --url postgres://.. --path ./migrations create my_name
# apply all non applied migrations
dbmigrate --url postgres://.. --path ./migrations up
# un-apply all migrations
dbmigrate --url postgres://.. --path ./migrations down
# redo the last migration
dbmigrate --url postgres://.. --path ./migrations redo
# see list of migrations and which one is currently applied
dbmigrate --url postgres://.. --path ./migrations status
```

The format of the migration files is the following:
```bash
0001.initial_db.up.sql
0001.initial_db.down.sql
```

## TODO

- find a way to do integration testing on travis + rust (use a python script?)

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
