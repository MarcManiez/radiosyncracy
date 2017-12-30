use iron::prelude::*;
use iron::status;
use urlencoded::{UrlEncodedBody, UrlEncodedQuery};

use ::models::track::Track;
use super::utils::*;
use ::views::api::tracks;

pub fn create(req: &mut Request) -> IronResult<Response> {
    let request_body = req.get_ref::<UrlEncodedBody>().expect("Failed to fetch query params.");

    if let Some(response) = require_params(request_body, vec!["link"]) {
        return Ok(response)
    }

    let length = match request_body.get("length") {
        Some(length) => Some(length[0].parse::<i32>().unwrap()),
        None => None,
    };
    let link = &request_body.get("link").unwrap()[0];
    let name = match request_body.get("name") {
        Some(name) => Some(name[0].as_str()),
        None => None,
    };

    match Track::create(length, link, name) {
        Ok(track) => Ok(Response::with((status::Created, tracks::create::render(&track)))),
        Err(error) => Ok(Response::with((status::InternalServerError, error))),
    }
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let request_params = req.get_ref::<UrlEncodedQuery>().expect("Failed to fetch query params.");

    if let Some(response) = require_params(request_params, vec!["id"]) {
        return Ok(response)
    }

    let id = request_params.get("id").unwrap()[0].parse::<i32>().unwrap();

    match Track::delete(id) {
        Ok(track) => Ok(Response::with((status::Ok, tracks::delete::render(&track)))),
        Err(error) => Ok(Response::with((status::InternalServerError, error))),
    }
}
