use diesel::prelude::*;
use iron::prelude::*;
use iron::status;
use serde_json;
use urlencoded::UrlEncodedBody;
use urlencoded::UrlEncodedQuery;

use std::ops::Deref;

use ::models::user::User;
use ::schema::users::dsl::*;
use super::utils::*;
use ::connection::POOL;
use ::views::api::users;

pub fn get_all_users(req: &mut Request) -> IronResult<Response> {
    let query_params = req.get_ref::<UrlEncodedQuery>().expect("Failed to fetch query params.");
    let email_address = &query_params["email"][0];

    let database_connection = POOL.get().expect("Failed to fetch a connection.");

    let all_users = users.filter(email.eq(email_address))
        .load::<User>(database_connection.deref())
        .expect("Error loading users.");

    let serialized_users = serde_json::to_string(&all_users).expect("Failed to serialize users");
    Ok(Response::with((status::Ok, serialized_users)))
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let request_body = req.get_ref::<UrlEncodedBody>().expect("Failed to fetch query params.");

    if let Some(response) = require_params(request_body, vec!["username", "email", "password"]) {
        return Ok(response)
    }

    let user_name = &request_body.get("username").unwrap()[0];
    let user_email = &request_body.get("email").unwrap()[0];
    let user_password = &request_body.get("password").unwrap()[0];

    let new_user = User::new(user_name, user_email, user_password);

    match new_user.save() {
        Ok(saved_user) => Ok(Response::with((status::Created, users::create::render(&saved_user)))),
        Err(_) => Ok(Response::with((status::InternalServerError, "Failed to insert user in database."))),
    }
}
