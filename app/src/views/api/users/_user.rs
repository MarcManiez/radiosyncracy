use ::models::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct RenderedUser<'a> {
    pub id: i32,
    pub username: &'a str,
    pub email: &'a str,
}

pub fn format(user: &User) -> RenderedUser {
    RenderedUser {
        id: user.id,
        username: &user.username,
        email: &user.email,
    }
}