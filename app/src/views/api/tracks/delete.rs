use serde_json;

use ::models::track::Track;
use ::views::api::tracks::_track::format;

pub fn render(track: &Track) -> String {
    serde_json::to_string(&format(track)).unwrap()
}