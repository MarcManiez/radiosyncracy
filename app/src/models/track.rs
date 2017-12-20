use chrono::NaiveDateTime;
use diesel;
use diesel::LoadDsl;

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
    pub length: i32,
    pub link: &'a str,
    pub name: &'a str,
}

impl Track {
    pub fn new<'a>(length: i32, link: &'a str, name: &'a str) -> NewTrack<'a> {
        NewTrack {
            length,
            link,
            name
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
