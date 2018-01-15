use diesel::prelude::*;

use std::ops::Deref;

use ::connection::POOL;
use ::models::radio::Radio;
use ::schema::radios;
use ::tests::truncate_all_tables;
use ::tests::factories::radio::*;

#[test]
fn find() {
    truncate_all_tables();
    let radio = create_basic_radio();
    let found_radio = Radio::find(radio.id).unwrap().unwrap();

    assert_eq!(radio.id, found_radio.id);
}

#[test]
fn create() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let name = "Hits from the '80s";
    let _radio = Radio::create(None, name);
    let radio = radios::table.filter(radios::name.eq(name)).get_result::<Radio>(database_connection.deref()).unwrap();

    assert_eq!(radio.name, name);
}

#[test]
fn update() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let radio = create_basic_radio();
    let radio_name = "Hits from the '70s";

    let _updated_radio = radio.update(None, None, Some(radio_name), None);
    let updated_radio = radios::table.filter(radios::name.eq(radio_name)).get_result::<Radio>(database_connection.deref()).unwrap();

    assert_eq!(radio.id, updated_radio.id);
}

#[test]
fn delete() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let radio = create_basic_radio();
    let _deleted_user = Radio::delete(radio.id).unwrap().unwrap();
    let all_radios = radios::table.get_results::<Radio>(database_connection.deref()).unwrap();

    assert_eq!(all_radios.len(), 0);
}

#[test]
fn user() {
    truncate_all_tables();
    let (radio, user) = create_radio_with_user();
    let found_user = radio.user().expect("Error getting user");

    assert_eq!(user.username, found_user.username);
}
