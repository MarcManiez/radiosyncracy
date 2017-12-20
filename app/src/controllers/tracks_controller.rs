use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedBody;

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

    let new_track = Track::new(length, link, name);

    match new_track.save() {
        Ok(saved_track) => Ok(Response::with((status::Created, tracks::create::render(&saved_track)))),
        Err(_) => Ok(Response::with((status::InternalServerError, "Failed to insert user in database."))),
    }
}
