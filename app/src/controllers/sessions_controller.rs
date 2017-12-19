use cookie::Cookie as CookiePair;
use hyper::header::Cookie;
use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedQuery;

use ::models::user::{Identifier, User};
use super::utils::*;
use ::views::api::sessions::create;

pub fn create(req: &mut Request) -> IronResult<Response> {
    let query_params = req.get_ref::<UrlEncodedQuery>().expect("Failed to fetch query params.");

    if let Some(response) = require_params(query_params, vec!["password"]) {
        return Ok(response)
    }
    let supplied_password = &query_params.get("password").unwrap()[0];

    let identifier: Identifier;
    if let Some(username) = query_params.get("username") {
        identifier = Identifier::Username(&username[0]);
    } else {
        match query_params.get("email") {
            Some(email) => identifier = Identifier::Email(&email[0]),
            None => return Ok(Response::with((status::UnprocessableEntity, format!("Identifier parameter was absent")))),
        }
    }

    match User::authenticate(identifier, supplied_password) {
        Ok(user) => {
            let mut response = Response::with((status::Ok, create::render(&user)));
            response.headers.set(Cookie(vec![
                CookiePair::new("user".to_owned(), create::render(&user)).to_string()
            ]));
            Ok(response)
        },
        Err(error) => {
            println!("{}", error);
            Ok(Response::with((status::Unauthorized, format!("Incorrect username or password."))))
        },
    }
}