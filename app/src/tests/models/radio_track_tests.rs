use diesel::prelude::*;

use std::ops::Deref;

use ::connection::POOL;
use ::models::radio_track::RadioTrack;
use ::schema::radio_tracks;
use ::tests::truncate_all_tables;
use ::tests::factories::radio_track::*;

#[test]
fn find() {
    truncate_all_tables();
    let (radio_track, _radio, _track) = create_radio_track();
    let found_radio_track = RadioTrack::find(radio_track.id).unwrap().unwrap();

    assert_eq!(radio_track.id, found_radio_track.id);
}

#[test]
fn create() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let track_order = 1;
    let _radio_track = RadioTrack::create(None, None, Some(track_order));
    let radio_track = radio_tracks::table.filter(radio_tracks::track_order.eq(track_order)).get_result::<RadioTrack>(database_connection.deref()).unwrap();

    assert_eq!(radio_track.track_order.unwrap(), track_order);
}

#[test]
fn update() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let (radio_track, _radio, _track) = create_radio_track();
    let track_order = 1;

    let _updated_radio_track = radio_track.update(track_order);
    let updated_radio_track = radio_tracks::table.filter(radio_tracks::track_order.eq(track_order)).get_result::<RadioTrack>(database_connection.deref()).unwrap();

    assert_eq!(radio_track.id, updated_radio_track.id);
}

#[test]
fn delete() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let (radio_track, _radio, _track) = create_radio_track();
    let _deleted_radio_track = RadioTrack::delete(radio_track.id).unwrap().unwrap();
    let all_radio_tracks = radio_tracks::table.get_results::<RadioTrack>(database_connection.deref()).unwrap();

    assert_eq!(all_radio_tracks.len(), 0);
}
