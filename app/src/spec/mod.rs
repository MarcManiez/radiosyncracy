use diesel::Connection;

use std::env;

use ::connection::{DatabaseConnection, POOL};
use ::environment::*;

#[test]
fn test() {

}

fn truncate_all_tables(connection: &DatabaseConnection) {
    if get() != TEST { return; }
    connection.execute(
      "CREATE OR REPLACE FUNCTION truncate_tables(username IN VARCHAR) RETURNS void AS $$\n\
      DECLARE\n\
          statements CURSOR FOR\n\
              SELECT tablename FROM pg_tables\n\
              WHERE tableowner = username AND schemaname = 'public';\n\
      BEGIN\n\
          FOR stmt IN statements LOOP\n\
              EXECUTE 'TRUNCATE TABLE ' || quote_ident(stmt.tablename) || ' CASCADE;';\n\
          END LOOP;\n\
      END;\n\
      $$ LANGUAGE plpgsql;"
    ).unwrap();
    let database_username = env::var("DATABASE_USERNAME").expect("Failed to load database username.");
    connection.execute(&format!("SELECT truncate_tables('{}');", database_username)).unwrap();
}

pub fn prepare_test() {
    let database_connection = POOL.get().expect("Failed to fetch a connection.");
    truncate_all_tables(&database_connection);
    database_connection.begin_test_transaction().unwrap();
}

