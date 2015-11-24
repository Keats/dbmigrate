# dbmigrate

A tool to create and manage PostgreSQL migrations.

## Usage

```bash
# create a migration file
migrate --url postgres://.. --path ./migrations create my_name
# apply all non applied migrations
migrate --url postgres://.. --path ./migrations up
# rollback last migration
migrate --url postgres://.. --path ./migrations rollback
# go to specific migration
migrate --url postgres://.. --path ./migrations goto 0001
# see list of migrations and which one is currently applied
migrate --url postgres://.. --path ./migrations status
```

Format of the migration files
```bash
0001_initial_db.up.sql
0001_initial_db.down.sql
```
