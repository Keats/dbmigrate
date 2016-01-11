CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL,
  password TEXT NOT NULL
);

-- Used in tests: SELECT truncate_tables()
-- Will truncate all the public tables (ie ours) excluding the schema one
CREATE OR REPLACE FUNCTION truncate_tables() RETURNS void AS $$
DECLARE
  statements CURSOR FOR
    SELECT tablename FROM pg_tables
    WHERE schemaname = 'public' AND tablename != 'schema_migrations';
BEGIN
  FOR stmt IN statements LOOP
    EXECUTE 'TRUNCATE TABLE ' || quote_ident(stmt.tablename) || ' CASCADE;';
  END LOOP;
END;
$$ LANGUAGE plpgsql;
