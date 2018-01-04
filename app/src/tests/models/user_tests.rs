use diesel::prelude::*;

use std::ops::Deref;

use ::connection::POOL;
use ::models::user::User;
use ::schema::users;
use ::tests::truncate_all_tables;
use ::tests::factories::user::*;

#[test]
fn find() {
    truncate_all_tables();
    let user = create_basic_user();
    let found_user = User::find(user.id).unwrap().unwrap();

    assert_eq!(user.id, found_user.id);
}

#[test]
fn create() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let user_email = "classeman@topofthe.pop";
    let _user = User::create("GeorgeAbitbol", user_email, "rosebud");
    let user = users::table.filter(users::email.eq(user_email)).get_result::<User>(database_connection.deref()).unwrap();

    assert_eq!(user.email, user_email);
}

