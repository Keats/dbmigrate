# dbmigrate

[![Build Status](https://travis-ci.org/Keats/dbmigrate.svg)](https://travis-ci.org/Keats/dbmigrate)


A tool to create and manage SQL migrations.


## Databases supported

- Postgres

## Usage

Every call to dbmigrate requires 2 arguments: database url and migrations folder.
Those can be set through environment variables: DBMIGRATE_URL and DBMIGRATE_PATH.

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

- add colours to console output
- time how long it takes to run each migration and print it
- find a way to implement generic Driver trait initializer to support other databases
- find a way to do integration testing on travis + rust (use a python script?)
