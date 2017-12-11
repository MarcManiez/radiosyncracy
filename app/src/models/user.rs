extern crate chrono;

use diesel;
use diesel::LoadDsl;
use self::chrono::NaiveDateTime;

use std::ops::Deref;

use ::connection;
use ::schema::users;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn new<'a>(username: &'a str, email: &'a str, password: &'a str) -> NewUser<'a> {
        NewUser {
            username,
            email,
            password,
        }
    }
}

impl<'a> NewUser<'a> {
    pub fn save(&self) -> Result<User, diesel::result::Error> {
        let pool = connection::establish_connection_pool();
        let database_connection = pool.get().expect("Failed to fetch a connection.");

        diesel::insert_into(users::table)
            .values(self)
            .get_result(database_connection.deref())
    }
}
