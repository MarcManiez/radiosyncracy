use iron::prelude::*;
use iron::status;

use ::models::user;
use ::connection;

pub fn get_all_users(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "A bunch of users here at some point")))
    // let pool = connection::establish_connection_pool();
    // let connection = pool.get().expect("Failed to fetch a connection.");
    // let all_users = users.load::<User>(connection.deref()).expect("Error loading users.");
}