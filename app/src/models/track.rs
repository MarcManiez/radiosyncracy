use chrono::NaiveDateTime;
use diesel;
use diesel::LoadDsl;
use regex::Regex;

use std::ops::Deref;

use ::connection::POOL;
use ::schema::tracks;

#[derive(Debug, Deserialize, Queryable, Serialize)]
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
}

impl<'a> NewTrack<'a> {
    pub fn save(&self) -> Result<Track, diesel::result::Error> {
        let database_connection = POOL.get().expect("Failed to fetch a connection.");

        diesel::insert_into(tracks::table)
            .values(self)
            .get_result(database_connection.deref())
    }
}
