use chrono::prelude::*;
use diesel;
use diesel::{FindDsl, LoadDsl, OptionalExtension};

use std::ops::Deref;

use ::connection::POOL;
use ::schema::radio_tracks;
use super::radio::{Radio};
use super::track::{Track};
use super::utils::{Deletable, print};

#[derive(AsChangeset, Debug, Deserialize, Identifiable, PartialEq, Queryable, Serialize)]
pub struct RadioTrack {
    pub id: i32,
    pub track_id: Option<i32>,
    pub radio_id: Option<i32>,
    pub track_order: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[table_name="radio_tracks"]
pub struct NewRadioTrack {
    pub track_id: Option<i32>,
    pub radio_id: Option<i32>,
    pub track_order: Option<i32>,
}

#[derive(AsChangeset, Debug)]
#[table_name="radio_tracks"]
struct RadioTrackUpdater {
    pub track_id: Option<i32>,
    pub radio_id: Option<i32>,
    pub track_order: Option<i32>,
    pub updated_at: NaiveDateTime,
}

impl RadioTrack {
    pub fn new(track_id: Option<i32>, radio_id: Option<i32>, track_order: Option<i32>) -> Result<NewRadioTrack, String> {
        match RadioTrack::validate(track_id, radio_id, track_order) {
            Some(error) => Err(format!("Error validating radio_track: {}", error)),
            None => Ok(NewRadioTrack {
                track_id,
                radio_id,
                track_order
            }),
        }
    }

    pub fn create<'a>(track_id: Option<i32>, radio_id: Option<i32>, track_order: Option<i32>) -> Result<RadioTrack, String> {
        match RadioTrack::new(track_id, radio_id, track_order) {
            Ok(new_radio_track) =>
                match new_radio_track.save() {
                    Ok(radio_track) => Ok(radio_track),
                    Err(error) => Err(format!("Error saving radio_track to database: {:?}", error)),
                }
            Err(error) => Err(error)
        }
    }

    pub fn validate<'a>(_track_id: Option<i32>, _radio_id: Option<i32>, _track_order: Option<i32>) -> Option<String> {
        // TODO: validate that track_id and radio_id are present (and potentially valid)
        // TODO: validate that track_order > 0
        // TODO: validate that track_order <= number of tracks on radio
        None
    }

    pub fn find(id: i32) -> Result<Option<RadioTrack>, String> {
        let database_connection = POOL.get().expect("Failed to fetch a connection");

        match print(radio_tracks::table.find(id)).get_result(database_connection.deref()).optional() {
            Ok(radio_track) => Ok(radio_track),
            Err(error) => Err(format!("Error finding radio_track: {:?}", error))
        }
    }

    pub fn update<'a>(&'a self, track_id: Option<i32>, radio_id: Option<i32>, track_order: Option<i32>) -> Result<RadioTrack, String> {
        if let Some(error) = RadioTrack::validate(track_id, radio_id, track_order) {
            return Err(format!("Error validating radio_track: {}", error))
        }

        let database_connection = POOL.get().expect("Failed to fetch a connection");
        let updated_radio_track = RadioTrackUpdater {
            track_id,
            radio_id,
            track_order,
            updated_at: Utc::now().naive_utc(),
        };
        let radio_track_update_query = diesel::update(radio_tracks::table.find(self.id)).set(&updated_radio_track);
        match print(radio_track_update_query).get_result(database_connection.deref()) {
            Ok(radio_track) => Ok(radio_track),
            Err(error) => Err(format!("Error updating radio_track: {:?}", error)),
        }
    }

    pub fn delete(id: i32) -> Result<Option<RadioTrack>, String> {
        let database_connection = POOL.get().expect("Failed to fetch a connection");

        match print(diesel::delete(radio_tracks::table.find(id))).get_result(database_connection.deref()).optional() {
            Ok(radio_track) => Ok(radio_track),
            Err(error) => Err(format!("Error deleting radio_track: {:?}", error)),
        }
    }

    pub fn radio<'a>(&'a self) -> Option<Radio> {
        match self.radio_id {
            Some(radio_id) => Radio::find(radio_id).expect("Error retrieving radio"),
            None => None,
        }
    }

    pub fn track<'a>(&'a self) -> Option<Track> {
        match self.track_id {
            Some(track_id) => Track::find(track_id).expect("Error retrieving radio"),
            None => None,
        }
    }
}

impl Deletable for RadioTrack {
    fn delete(&self) -> Result<Option<RadioTrack>, String> {
        match RadioTrack::delete(self.id) {
            Ok(deleted) => Ok(deleted),
            Err(error) => Err(error),
        }
    }
}

impl NewRadioTrack {
    pub fn save(&self) -> Result<RadioTrack, diesel::result::Error> {
        let database_connection = POOL.get().expect("Failed to fetch a connection.");

        print(diesel::insert_into(radio_tracks::table).values(self)).get_result(database_connection.deref())
    }
}
