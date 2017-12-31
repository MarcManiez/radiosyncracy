use chrono::prelude::*;
use diesel;
use diesel::{FindDsl, LoadDsl, OptionalExtension};
use regex::Regex;

use std::ops::Deref;

use ::connection::POOL;
use ::schema::tracks;
use super::utils::{Deletable, print};

#[derive(AsChangeset, Debug, Deserialize, Identifiable, Queryable, Serialize)]
pub struct Track {
    pub id: i32,
    pub length: Option<i32>,
    pub link: String,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[table_name="tracks"]
pub struct NewTrack<'a> {
    pub length: Option<i32>,
    pub link: &'a str,
    pub name: Option<&'a str>,
}

#[derive(AsChangeset, Debug)]
#[table_name="tracks"]
struct TrackUpdater<'a> {
    pub length: Option<i32>,
    pub link: Option<&'a str>,
    pub name: Option<&'a str>,
    pub updated_at: NaiveDateTime,
}

impl Track {
    pub fn new<'a>(length: Option<i32>, link: &'a str, name: Option<&'a str>) -> Result<NewTrack<'a>, String> {
        match Track::validate(length, Some(link), name) {
            Some(error) => Err(format!("Error validating track: {}", error)),
            None => Ok(NewTrack {
                length,
                link,
                name
            }),
        }
    }

    pub fn create<'a>(length: Option<i32>, link: &'a str, name: Option<&'a str>) -> Result<Track, String> {
        match Track::new(length, link, name) {
            Ok(new_track) =>
                match new_track.save() {
                    Ok(track) => Ok(track),
                    Err(error) => Err(format!("Error saving track to database: {:?}", error)),
                }
            Err(error) => Err(error)
        }
    }

    pub fn validate<'a>(_length: Option<i32>, link: Option<&'a str>, _name: Option<&'a str>) -> Option<String> {
        // TODO: to scale this, iterate over a vector of validation closures (one for each rule) and return the first one that
        // gives us a string (that means something went wrong in the validation process and we should propagate the message)
        if let Some(url) = link {
            let youtube_url_regex = Regex::new("https?:\x2F\x2F(w{3}\x2E)?youtu(be\x2Ecom|\x2Ebe)\x2F.+").expect("Failed to parse regex");
            if !youtube_url_regex.is_match(url) {
                return Some(String::from("Link is not a youtube URL."))
            }
        }
        None
    }

    pub fn find(id: i32) -> Result<Option<Track>, String> {
        let database_connection = POOL.get().expect("Failed to fetch a connection.");

        match print(tracks::table.find(id)).get_result(database_connection.deref()).optional() {
            Ok(track) => Ok(track),
            Err(error) => Err(format!("Error finding track: {:?}", error))
        }
    }

    pub fn update<'a>(
        &'a self,
        length: Option<i32>,
        link: Option<&'a str>,
        name: Option<&'a str>
    ) -> Result<Track, String> {
        if let Some(error) = Track::validate(length, link, name) {
            return Err(format!("Error validating track: {}", error))
        }

        let database_connection = POOL.get().expect("Failed to fetch a connection.");
        let updated_track = TrackUpdater {
            length,
            link,
            name,
            updated_at: Utc::now().naive_utc(),
        };
        let track_update_query = diesel::update(tracks::table.find(self.id)).set(&updated_track);
        match print(track_update_query).get_result(database_connection.deref()) {
            Ok(track) => Ok(track),
            Err(error) => Err(format!("Error updating track: {:?}", error)),
        }
    }

    pub fn delete(id: i32) -> Result<Option<Track>, String> {
        let database_connection = POOL.get().expect("Failed to fetch a connection.");

        match print(diesel::delete(tracks::table.find(id))).get_result(database_connection.deref()).optional() {
            Ok(track) => Ok(track),
            Err(error) => Err(format!("Error deleting track: {:?}", error)),
        }
    }
}

impl Deletable for Track {
    fn delete(&self) -> Result<Option<Track>, String> {
        match Track::delete(self.id) {
            Ok(deleted) => Ok(deleted),
            Err(error) => Err(error),
        }
    }
}

impl<'a> NewTrack<'a> {
    pub fn save(&self) -> Result<Track, diesel::result::Error> {
        let database_connection = POOL.get().expect("Failed to fetch a connection.");

        print(diesel::insert_into(tracks::table).values(self)).get_result(database_connection.deref())
    }
}
