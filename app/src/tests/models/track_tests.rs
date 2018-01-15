use diesel::prelude::*;

use std::ops::Deref;

use ::connection::POOL;
use ::models::track::Track;
use ::schema::tracks;
use ::tests::truncate_all_tables;
use ::tests::factories::track::*;

#[test]
fn find() {
    truncate_all_tables();
    let track = create_basic_track();
    let found_track = Track::find(track.id).unwrap().unwrap();

    assert_eq!(track.id, found_track.id);
}

#[test]
fn create() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let link = "https://www.youtube.com/watch?v=Zmfvx4DuBPI";
    let _track = Track::create(None, link, None);
    let track = tracks::table.filter(tracks::link.eq(link)).get_result::<Track>(database_connection.deref()).unwrap();

    assert_eq!(track.link, link);
}

#[test]
fn update() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let track = create_basic_track();
    let track_name = "People First";

    let _updated_track = track.update(None, None, Some(track_name));
    let updated_track = tracks::table.filter(tracks::name.eq(track_name)).get_result::<Track>(database_connection.deref()).unwrap();

    assert_eq!(track.id, updated_track.id);
}

#[test]
fn delete() {
    truncate_all_tables();
    let database_connection = POOL.get().expect("Failed to fetch a connection");
    let track = create_basic_track();
    let _deleted_track = Track::delete(track.id).unwrap().unwrap();
    let all_tracks = tracks::table.get_results::<Track>(database_connection.deref()).unwrap();

    assert_eq!(all_tracks.len(), 0);
}
