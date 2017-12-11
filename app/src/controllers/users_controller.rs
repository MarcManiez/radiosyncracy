use diesel::prelude::*;
use iron::prelude::*;
use iron::status;
use serde_json;
use urlencoded::UrlEncodedBody;
use urlencoded::UrlEncodedQuery;

use std::ops::Deref;

use ::models::user::User;
use ::schema::users::dsl::*;
use ::connection;

pub fn get_all_users(req: &mut Request) -> IronResult<Response> {
    let query_params = req.get_ref::<UrlEncodedQuery>().expect("Failed to fetch query params.");
    let email_address = &query_params["email"][0];

    let pool = connection::establish_connection_pool();
    let database_connection = pool.get().expect("Failed to fetch a connection.");

    let all_users = users.filter(email.eq(email_address))
        .load::<User>(database_connection.deref())
        .expect("Error loading users.");

    let serialized_users = serde_json::to_string(&all_users).expect("Failed to serialize users");
    Ok(Response::with((status::Ok, serialized_users)))
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let query_params = req.get_ref::<UrlEncodedBody>().expect("Failed to fetch query params.");
    let required_params = ["username", "email", "password"];

    for &param in &required_params {
        if let None = query_params.get(param) {
            return Ok(Response::with((status::UnprocessableEntity, format!("Required parameter '{}' was absent", param))));
        }
    }

    let user_name = &query_params.get("username").unwrap()[0];
    let user_email = &query_params.get("email").unwrap()[0];
    let user_password = &query_params.get("password").unwrap()[0];

    let new_user = User::new(user_name, user_email, user_password);
    new_user.save();

    Ok(Response::with((status::Ok, "Just testing")))
}