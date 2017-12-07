extern crate chrono;
use self::chrono::NaiveDateTime;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub password: String,
}
