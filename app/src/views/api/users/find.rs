use serde_json;

use ::models::user::User;
use ::views::api::users::_user::format;

pub fn render(user: &User) -> String {
    serde_json::to_string(&format(user)).unwrap()
}