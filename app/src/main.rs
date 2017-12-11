#![recursion_limit="128"]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
#[macro_use] extern crate serde_derive;
extern crate bcrypt;
extern crate dotenv;
extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate urlencoded;

pub mod models;
pub mod schema;
pub mod views;
mod api_routes;
mod connection;
mod controllers;
use iron::prelude::*;

fn main() {
    let routes = api_routes::get_routes();

    let _server = Iron::new(routes).http("localhost:3000").unwrap();
    println!("On 3000");
}
