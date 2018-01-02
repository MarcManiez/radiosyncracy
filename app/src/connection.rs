extern crate r2d2;
extern crate r2d2_diesel;

use diesel::PgConnection;
use dotenv::dotenv;
use self::r2d2::{Error, Pool, PooledConnection};
use self::r2d2_diesel::ConnectionManager;

use std::env;

use ::environment::*;

type Manager = ConnectionManager<PgConnection>;
pub type DatabaseConnection = PooledConnection<Manager>;

pub const POOL: ConnectionPool = ConnectionPool { pool: None };

pub struct ConnectionPool {
    pool: Option<Pool<Manager>>
}

impl ConnectionPool {
    pub fn get(mut self) -> Result<DatabaseConnection, Error> {
        if let None = self.pool {
            self.instantiate_connection_pool();
        }
        self.pool.expect("Failed to fetch pool").get()
    }

    fn instantiate_connection_pool(&mut self) {
        let database_url = get_database_url().expect("Database url must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        self.pool = Some(r2d2::Pool::builder().max_size(15).build(manager).expect("Failed to create connection pool."))
    }
}

fn get_database_url() -> Result<String, env::VarError> {
    dotenv().ok();
    if get() == TEST {
        env::var("TEST_DATABASE_URL")
    } else {
        env::var("DATABASE_URL")
    }
}


