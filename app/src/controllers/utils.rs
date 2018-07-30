use iron::prelude::*;
use iron::status;

use std::collections::HashMap;

pub fn require_params(query_params: &HashMap<String, Vec<String>>, required_params: Vec<&str>) -> Result<HashMap<String, String>, Response> {
    let mut params = HashMap::new();
    for param in required_params {
        if let Some(fetched_value) = query_params.get(param) {
            params.insert(param.to_owned(), fetched_value[0].to_owned());
        } else {
            return Err(Response::with((status::UnprocessableEntity, format!("Required parameter '{}' was absent", param))));
        }
    }
    Ok(params)
}
