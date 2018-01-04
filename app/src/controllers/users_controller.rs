use iron::prelude::*;
use iron::status;
use urlencoded::{UrlEncodedBody, UrlEncodedQuery};

use ::models::user::User;
use super::utils::*;
use ::views::api::users;

pub fn create(req: &mut Request) -> IronResult<Response> {
    let request_body = req.get_ref::<UrlEncodedBody>().expect("Failed to fetch query params");

    if let Some(response) = require_params(request_body, vec!["username", "email", "password"]) {
        return Ok(response)
    }

    let username = &request_body.get("username").unwrap()[0];
    let email = &request_body.get("email").unwrap()[0];
    let supplied_password = &request_body.get("password").unwrap()[0];

    match User::create(username, email, supplied_password) {
        Ok(user) => Ok(Response::with((status::Created, users::create::render(&user)))),
        Err(error) => Ok(Response::with((status::InternalServerError, error))),
    }
}

pub fn find(req: &mut Request) -> IronResult<Response> {
    let request_params = req.get_ref::<UrlEncodedQuery>().expect("Failed to fetch query params");

    if let Some(response) = require_params(request_params, vec!["id"]) {
        return Ok(response)
    }

    let id = request_params.get("id").unwrap()[0].parse::<i32>().unwrap();
    match User::find(id) {
        Ok(user) => {
            match user {
                Some(found_user) => Ok(Response::with((status::Ok, users::find::render(&found_user)))),
                None => Ok(Response::with((status::NotFound, "User was not found."))),
            }
        },
        Err(error) => Ok(Response::with((status::InternalServerError, error))),
    }

}
