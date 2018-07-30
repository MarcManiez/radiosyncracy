use iron::prelude::*;
use iron::status;
use urlencoded::{UrlEncodedBody, UrlEncodedQuery};

use ::models::user::User;
use super::utils::*;
use ::views::api::users;

pub fn create(req: &mut Request) -> IronResult<Response> {
    let request_body = req.get_ref::<UrlEncodedBody>().expect("Failed to fetch query params");

    let params = match require_params(request_body, vec!["username", "email", "password"]) {
      Ok(params) => params,
      Err(response) => return Ok(response),
    };

    let username = params.get("username").unwrap();
    let email = params.get("email").unwrap();
    let supplied_password = params.get("password").unwrap();

    match User::create(&username, &email, &supplied_password) {
        Ok(user) => Ok(Response::with((status::Created, users::create::render(&user)))),
        Err(error) => Ok(Response::with((status::InternalServerError, error))),
    }
}

pub fn find(req: &mut Request) -> IronResult<Response> {
    let request_params = req.get_ref::<UrlEncodedQuery>().expect("Failed to fetch query params");

    if let Err(response) = require_params(request_params, vec!["id"]) {
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
