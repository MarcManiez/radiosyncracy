#![recursion_limit="128"]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
extern crate dotenv;
extern crate iron;

pub mod models;
pub mod schema;
mod connection;

use std::ops::Deref;

use ::models::user::User;
use ::schema::users::dsl::*;
use diesel::prelude::*;
use iron::prelude::*;
use iron::status;

fn main() {

    let pool = connection::establish_connection_pool();
    let connection = pool.get().expect("Failed to fetch a connection.");
    let all_users = users.load::<User>(connection.deref()).expect("Error loading users.");

    println!("the user: {:?}", all_users[0]);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    let _server = Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
