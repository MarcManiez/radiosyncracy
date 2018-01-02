use ::models::user::User;

pub fn create() -> User {
    User::create("username", "valid@email.com", "password").unwrap()
}
