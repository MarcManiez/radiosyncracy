use ::models::user::User;
use ::models::radio::Radio;
use super::user::create_basic_user;

pub fn create_basic_radio() -> Radio {
    Radio::create(None, "Hits from the '90s").unwrap()
}

pub fn create_radio_with_user() -> (Radio, User) {
    let user = create_basic_user();
    let radio = Radio::create(Some(user.id), "Hits from the '90s").unwrap();
    (radio, user)
}
