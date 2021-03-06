#![recursion_limit="128"]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
#[macro_use] extern crate serde_derive;
extern crate bcrypt;
extern crate chrono;
extern crate cookie;
extern crate dotenv;
extern crate hyper;
extern crate iron;
extern crate rand;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate urlencoded;

#[cfg(test)] mod tests;
pub mod models;
pub mod schema;
pub mod views;
mod api_routes;
mod environment;
mod connection;
mod controllers;

use dotenv::dotenv;
use iron::prelude::*;

fn main() {
    dotenv().ok();
    let routes = api_routes::get_routes();

    let _server = Iron::new(routes).http("localhost:3000").unwrap();
    println!("On 3000");
}
