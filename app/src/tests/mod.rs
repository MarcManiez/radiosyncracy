use diesel::Connection;

use std::env;

use ::connection::POOL;
use ::environment::*;

pub mod factories;
mod models;

pub fn truncate_all_tables() {
    if get() != TEST { return; }
    let connection = POOL.get().expect("Failed to fetch a connection.");
    connection.execute(
      "CREATE OR REPLACE FUNCTION truncate_tables(username IN VARCHAR) RETURNS void AS $$\n\
      DECLARE\n\
          statements CURSOR FOR\n\
              SELECT tablename FROM pg_tables\n\
              WHERE tableowner = username AND schemaname = 'public' AND tablename != '__diesel_schema_migrations';\n\
      BEGIN\n\
          FOR stmt IN statements LOOP\n\
              EXECUTE 'TRUNCATE TABLE ' || quote_ident(stmt.tablename) || ' CASCADE;';\n\
              EXECUTE 'ALTER SEQUENCE ' || quote_ident(stmt.tablename) || '_id_seq RESTART WITH 1;';\n\
              EXECUTE 'UPDATE ' || quote_ident(stmt.tablename) || ' SET id = DEFAULT';\n\
          END LOOP;\n\
      END;\n\
      $$ LANGUAGE plpgsql;"
    ).unwrap();
    let database_username = env::var("DATABASE_USERNAME").expect("Failed to load database username.");
    connection.execute(&format!("SELECT truncate_tables('{}');", database_username)).unwrap();
}
