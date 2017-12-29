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
        let youtube_url_regex = Regex::new("https?:\x2F\x2F(w{3}\x2E)?youtu(be\x2Ecom|\x2Ebe)\x2F.+").expect("Failed to parse regex");
        if youtube_url_regex.is_match(link) {
            Ok(NewTrack {
                length,
                link,
                name
            })
        } else {
            Err(String::from("Link is not a youtube URL."))
        }
    }

    pub fn create<'a>(length: Option<i32>, link: &'a str, name: Option<&'a str>) -> Result<Track, String> {
        match Track::new(length, link, name) {
            Ok(new_track) =>
                match new_track.save() {
                    Ok(track) => Ok(track),
                    Err(error) => Err(format!("Error saving track to database: {:?}", error))
                }
            Err(error) => Err(error)
        }
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
