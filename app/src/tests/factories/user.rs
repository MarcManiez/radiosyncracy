use ::models::user::User;

pub fn create_basic_user() -> User {
    User::create("username", "valid@email.com", "password").unwrap()
}
