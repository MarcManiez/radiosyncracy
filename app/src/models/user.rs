extern crate chrono;

use bcrypt::{DEFAULT_COST, hash};
use diesel;
use diesel::LoadDsl;
use rand::{thread_rng, Rng};
use self::chrono::NaiveDateTime;

use std::ops::Deref;

use ::connection::POOL;
use ::schema::users;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub password: String,
    pub password_salt: String,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: String,
    pub password_salt: String,
}

impl User {
    pub fn new<'a>(username: &'a str, email: &'a str, supplied_password: &'a str) -> NewUser<'a> {
        let password_salt: String = thread_rng().gen_ascii_chars().take(10).collect();
        let password = User::hash_salted_password(supplied_password, &password_salt);
        NewUser {
            username,
            email,
            password,
            password_salt
        }
    }

    fn hash_salted_password(supplied_password: &str, password_salt: &str) -> String {
        let salted_password = format!("{}{}", password_salt, supplied_password);
        hash(&salted_password, DEFAULT_COST).expect("Failed to hash password.")
    }
}

impl<'a> NewUser<'a> {
    pub fn save(&self) -> Result<User, diesel::result::Error> {
        let database_connection = POOL.get().expect("Failed to fetch a connection.");

        diesel::insert_into(users::table)
            .values(self)
            .get_result(database_connection.deref())
    }
}
