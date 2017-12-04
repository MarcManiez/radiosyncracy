extern crate r2d2;
extern crate r2d2_diesel;

use std::env;
use std::thread;

use diesel::PgConnection;
use dotenv::dotenv;
use self::r2d2::Pool;
use self::r2d2_diesel::ConnectionManager;

pub fn initialize_pool() {
    let pool = establish_connection_pool();

    for _ in 0..10 {
        let pool = pool.clone();
        thread::spawn(move || {
            let connection = pool.get();

            assert!(connection.is_ok());
        });
    }
}

pub fn establish_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
