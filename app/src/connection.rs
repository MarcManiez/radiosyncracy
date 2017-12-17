extern crate r2d2;
extern crate r2d2_diesel;

use std::env;

use diesel::PgConnection;
use dotenv::dotenv;
use self::r2d2::{Error, Pool, PooledConnection};
use self::r2d2_diesel::ConnectionManager;

pub const POOL: ConnectionPool = ConnectionPool { pool: None };

pub struct ConnectionPool {
    pool: Option<Pool<ConnectionManager<PgConnection>>>
}

impl ConnectionPool {
    pub fn get(mut self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        if let None = self.pool {
            self.pool = Some(ConnectionPool::instantiate_connection_pool());
        }
        self.pool.expect("1").clone().get()
    }

    fn instantiate_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder().build(manager).expect("Failed to create connection pool.")
    }
}


