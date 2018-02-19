use diesel::prelude::*;

use std::ops::Deref;

use ::connection::POOL;
use ::models::radio::Radio;
use ::schema::radios;
use ::tests::truncate_all_tables;
use ::tests::factories::radio::*;
use ::tests::factories::radio_track::*;
use ::tests::factories::track::*;

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
    let mut radio = create_basic_radio();
    let radio_name = "Hits from the '70s";

    radio = radio.update(None, None, Some(radio_name), None).expect("Error updating radio");
    let updated_radio = radios::table.filter(radios::name.eq(radio_name)).get_result::<Radio>(database_connection.deref()).unwrap();

    assert_eq!(radio.id, updated_radio.id);
}

#[test]
fn delete() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let radio = create_basic_radio();
    let _deleted_radio = Radio::delete(radio.id).unwrap().unwrap();
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

#[test]
fn tracks() {
    truncate_all_tables();
    let (first_radio_track, radio, track) = create_radio_track();
    let second_radio_track = create_basic_radio_track();
    let second_radio_track = second_radio_track.update(Some(radio.id), Some(track.id), Some(2)).expect("Error updating radio track");
    let radio_tracks = radio.tracks().expect("Error fetching radio tracks");

    assert_eq!(radio_tracks, vec![first_radio_track, second_radio_track]);
}

#[test]
fn add_track() {
    truncate_all_tables();
    let radio = create_basic_radio();
    let track = create_basic_track();
    let radio_track = radio.add_track(&track).expect("Error adding radio track");

    assert_eq!(radio_track.track_order.unwrap(), 1);
    assert_eq!(radio_track.track_id.unwrap(), track.id);
    assert_eq!(radio_track.radio_id.unwrap(), radio.id);
}

#[test]

fn current_track() {
    truncate_all_tables();
    let mut radio = create_basic_radio();
    let track_1 = create_basic_track();
    let track_2 = create_basic_track();
    let track_3 = create_basic_track();
    let _ = radio.add_track(&track_1).expect("Error adding radio track_1");
    let radio_track2 = radio.add_track(&track_2).expect("Error adding radio track_2");
    let _ = radio.add_track(&track_3).expect("Error adding radio track_3");
    radio = radio.update(None, Some(2), None, None).expect("Error updating radio");
    let current_radio_track = radio.current_track().expect("Error fetching current track");

    assert_eq!(current_radio_track, radio_track2);
}

#[test]
fn current_track_without_any_track_on_radio() {
    truncate_all_tables();
    let radio = create_basic_radio();
    let current_radio_track = radio.current_track();

    assert_eq!(current_radio_track, None);
}
