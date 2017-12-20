extern crate r2d2;
extern crate r2d2_diesel;

use std::env;

use diesel::PgConnection;
use dotenv::dotenv;
use self::r2d2::{Error, Pool, PooledConnection};
use self::r2d2_diesel::ConnectionManager;

type Manager = ConnectionManager<PgConnection>;

pub const POOL: ConnectionPool = ConnectionPool { pool: None };

pub struct ConnectionPool {
    pool: Option<Pool<Manager>>
}

impl ConnectionPool {
    pub fn get(mut self) -> Result<PooledConnection<Manager>, Error> {
        if let None = self.pool {
            self.instantiate_connection_pool();
        }
        self.pool.expect("Failed to fetch pool").get()
    }

    fn instantiate_connection_pool(&mut self) {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        self.pool = Some(r2d2::Pool::builder().max_size(15).build(manager).expect("Failed to create connection pool."))
    }
}


