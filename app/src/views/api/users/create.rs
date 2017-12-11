use serde_json;

use ::models::user::User;

pub fn render(user: User) -> String {
    serde_json::to_string(&user).unwrap()
}