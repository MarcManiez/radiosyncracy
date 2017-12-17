use iron::prelude::*;
use iron::status;

use std::collections::HashMap;

pub fn validate_params(query_params: &HashMap<String, Vec<String>>, required_params: Vec<&str>) -> Option<Response> {
    for param in required_params {
        if let None = query_params.get(param) {
            return Some(Response::with((status::UnprocessableEntity, format!("Required parameter '{}' was absent", param))));
        }
    }
    None
}