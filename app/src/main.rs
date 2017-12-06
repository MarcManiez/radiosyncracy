#![recursion_limit="128"]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
extern crate dotenv;
extern crate iron;

pub mod models;
pub mod schema;
mod api_routes;
mod connection;
mod controllers;

use std::ops::Deref;

use ::models::user::User;
use ::schema::users::dsl::*;
use diesel::prelude::*;
use iron::prelude::*;

fn main() {
    let pool = connection::establish_connection_pool();
    let connection = pool.get().expect("Failed to fetch a connection.");
    let all_users = users.load::<User>(connection.deref()).expect("Error loading users.");

    println!("the user: {:?}", all_users[0]);

    let routes = api_routes::get_routes();

    let _server = Iron::new(routes).http("localhost:3000").unwrap();
    println!("On 3000");
}
