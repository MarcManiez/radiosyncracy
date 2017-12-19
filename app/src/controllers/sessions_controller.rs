use cookie::Cookie as CookiePair;
use hyper::header::Cookie;
use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedBody;

use ::models::user::{Identifier, User};
use super::utils::*;
use ::views::api::sessions::create;

pub fn create(req: &mut Request) -> IronResult<Response> {
    let request_body = req.get_ref::<UrlEncodedBody>().expect("Failed to fetch query params.");

    if let Some(response) = require_params(request_body, vec!["password"]) {
        return Ok(response)
    }
    let supplied_password = &request_body.get("password").unwrap()[0];

    let identifier: Identifier;
    if let Some(username) = request_body.get("username") {
        identifier = Identifier::Username(&username[0]);
    } else {
        match request_body.get("email") {
            Some(email) => identifier = Identifier::Email(&email[0]),
            None => return Ok(Response::with((status::UnprocessableEntity, format!("Identifier parameter was absent")))),
        }
    }

    match User::authenticate(identifier, supplied_password) {
        Ok(user) => {
            let mut response = Response::with((status::Ok, create::render(&user)));
            response.headers.set(Cookie(vec![
                CookiePair::new("foo".to_owned(), "bar".to_owned()).to_string()
            ]));
            Ok(response)
        },
        Err(error) => {
            println!("{}", error);
            Ok(Response::with((status::Unauthorized, format!("Incorrect username or password."))))
        },
    }
}