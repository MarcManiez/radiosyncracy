use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::prelude::*;
use diesel;
use diesel::prelude::*;
use diesel::{OptionalExtension, result};
use rand::{thread_rng, Rng};

use std::ops::Deref;

use ::connection::POOL;
use ::schema::users;
use super::utils::{Deletable, print};

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

#[derive(AsChangeset, Debug)]
#[table_name="users"]
struct UserUpdater<'a> {
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a String>,
    pub updated_at: NaiveDateTime,
}

pub enum Identifier<'a> {
    Email(&'a str),
    Username(&'a str),
}

impl User {
    pub fn new<'a>(username: &'a str, email: &'a str, supplied_password: &'a str) -> Result<NewUser<'a>, String> {
        match User::validate(Some(username), Some(email), Some(supplied_password)) {
            Some(error) => Err(format!("Error validating user: {}", error)),
            None => {
                let password_salt: String = thread_rng().gen_ascii_chars().take(10).collect();
                let password = User::hash_salt_and_password(supplied_password, &password_salt);
                Ok(NewUser {
                    username,
                    email,
                    password,
                    password_salt
                })
            },
        }
    }

    pub fn validate<'a>(_username: Option<&'a str>, _email: Option<&'a str>, _password: Option<&'a str>) -> Option<String> {
        None
    }

    pub fn create<'a>(username: &'a str, email: &'a str, supplied_password: &'a str) -> Result<User, String> {
        match User::new(username, email, supplied_password) {
            Ok(new_user) =>
                match new_user.save() {
                    Ok(user) => Ok(user),
                    Err(error) => Err(format!("Error saving user to database: {:?}", error))
                }
            Err(error) => Err(error)
        }
    }

    pub fn find(id: i32) -> Result<Option<User>, String> {
        let database_connection = POOL.get().expect("Failed to fetch a connection");

        match print(users::table.find(id)).get_result(database_connection.deref()).optional() {
            Ok(user) => Ok(user),
            Err(error) => Err(format!("Error finding user: {:?}", error))
        }
    }

    pub fn update<'a>(
        &'a self,
        username: Option<&'a str>,
        email: Option<&'a str>,
        supplied_password: Option<&'a str>
    ) -> Result<User, String> {
        if let Some(error) = User::validate(username, email, supplied_password) {
            return Err(format!("Error validating user: {}", error))
        }

        let password;
        match supplied_password {
            Some(user_supplied_password) => password = Some(User::hash_salt_and_password(user_supplied_password, &self.password_salt)),
            None => password = None,
        }

        let database_connection = POOL.get().expect("Failed to fetch a connection");
        let updated_user = UserUpdater {
            username,
            email,
            password: password.as_ref(),
            updated_at: Utc::now().naive_utc(),
        };
        let user_update_query = diesel::update(users::table.find(self.id)).set(&updated_user);
        match print(user_update_query).get_result(database_connection.deref()) {
            Ok(user) => Ok(user),
            Err(error) => Err(format!("Error updating user: {:?}", error)),
        }
    }

    pub fn delete(id: i32) -> Result<Option<User>, String> {
        let database_connection = POOL.get().expect("Failed to fetch a connection");

        match print(diesel::delete(users::table.find(id))).get_result(database_connection.deref()).optional() {
            Ok(user) => Ok(user),
            Err(error) => Err(format!("Error deleting track: {:?}", error)),
        }
    }

    pub fn authenticate(identifier: Identifier, supplied_password: &str) -> Result<User, result::Error> {
        use ::schema::users::dsl::*;
        let database_connection = POOL.get().expect("Failed to fetch a connection");

        let user = match identifier {
            Identifier::Email(submitted_email) => {
                users.filter(email.eq(submitted_email)).first::<User>(database_connection.deref())
            },
            Identifier::Username(submitted_username) => {
                users.filter(username.eq(submitted_username)).first::<User>(database_connection.deref())
            },
        };

        if let Ok(ref found_user) = user {
            let salted_supplied_password = format!("{}{}", found_user.password_salt, supplied_password);
            if !verify(&salted_supplied_password, &found_user.password).unwrap() {
                return Err(result::Error::NotFound)
            }
        }
        user
    }

    fn hash_salt_and_password(supplied_password: &str, password_salt: &str) -> String {
        let salted_password = format!("{}{}", password_salt, supplied_password);
        hash(&salted_password, DEFAULT_COST).expect("Failed to hash password")
    }
}

impl Deletable for User {
    fn delete(&self) -> Result<Option<User>, String> {
        match User::delete(self.id) {
            Ok(deleted) => Ok(deleted),
            Err(error) => Err(error),
        }
    }
}

impl<'a> NewUser<'a> {
    pub fn save(&self) -> Result<User, result::Error> {
        let database_connection = POOL.get().expect("Failed to fetch a connection");

        print(diesel::insert_into(users::table).values(self)).get_result(database_connection.deref())
    }
}
