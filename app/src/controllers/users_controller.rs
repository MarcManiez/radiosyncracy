use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedBody;

use ::models::user::User;
use super::utils::*;
use ::views::api::users;

pub fn create(req: &mut Request) -> IronResult<Response> {
    let request_body = req.get_ref::<UrlEncodedBody>().expect("Failed to fetch query params.");

    if let Some(response) = require_params(request_body, vec!["username", "email", "password"]) {
        return Ok(response)
    }

    let user_name = &request_body.get("username").unwrap()[0];
    let user_email = &request_body.get("email").unwrap()[0];
    let user_password = &request_body.get("password").unwrap()[0];

    let new_user = match User::new(user_name, user_email, user_password) {
        Ok(user) => user,
        Err(_) => return Ok(Response::with((status::InternalServerError, "Failed to instantiate user."))),
    };

    match new_user.save() {
        Ok(saved_user) => Ok(Response::with((status::Created, users::create::render(&saved_user)))),
        Err(_) => Ok(Response::with((status::InternalServerError, "Failed to insert user in database."))),
    }
}
