use chrono::prelude::*;
use diesel;
use diesel::{ExpressionMethods, FilterDsl, FindDsl, LoadDsl, OptionalExtension, OrderDsl};

use std::ops::Deref;

use ::connection::POOL;
use ::schema::radio_tracks;
use ::schema::radios;
use super::radio_track::RadioTrack;
use super::user::User;
use super::utils::{Deletable, print};

#[derive(AsChangeset, Debug, Deserialize, Identifiable, Queryable, Serialize)]
pub struct Radio {
    pub id: i32,
    pub user_id: Option<i32>,
    pub last_played_radio_track_number: Option<i32>,
    pub name: String,
    pub seconds_played_on_last_radio_track: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[table_name="radios"]
pub struct NewRadio<'a> {
    pub user_id: Option<i32>,
    pub last_played_radio_track_number: Option<i32>,
    pub name: &'a str,
    pub seconds_played_on_last_radio_track: Option<i32>,
}

#[derive(AsChangeset, Debug)]
#[table_name="radios"]
struct RadioUpdater<'a> {
    pub user_id: Option<i32>,
    pub last_played_radio_track_number: Option<i32>,
    pub name: Option<&'a str>,
    pub seconds_played_on_last_radio_track: Option<i32>,
    pub updated_at: NaiveDateTime,
}

impl Radio {
    pub fn new<'a>(user_id: Option<i32>, name: &'a str) -> Result<NewRadio<'a>, String> {
        match Radio::validate(user_id, None, Some(name), None) {
            Some(error) => Err(format!("Error validating radio: {}", error)),
            None => Ok(NewRadio {
                user_id,
                last_played_radio_track_number: None,
                name,
                seconds_played_on_last_radio_track: None,
            }),
        }
    }

    pub fn create<'a>(user_id: Option<i32>, name: &'a str) -> Result<Radio, String> {
        match Radio::new(user_id, name) {
            Ok(new_radio) =>
                match new_radio.save() {
                    Ok(radio) => Ok(radio),
                    Err(error) => Err(format!("Error saving radio to database: {:?}", error)),
                }
            Err(error) => Err(error)
        }
    }

    pub fn validate<'a>(
      _user_id: Option<i32>,
      _last_played_radio_track_number: Option<i32>,
      _name: Option<&'a str>,
      _seconds_played_on_last_radio_track: Option<i32>
    ) -> Option<String> {
      // TODO: validate lenght of radio name to 50 max
      // TODO: validate user_id: is for a valid user
      // TODO: validate last_played_radio_track_number: radio has tracks on it
      // TODO: validate last_played_radio_track_number: is > 0
      // TODO: validate last_played_radio_track_number: is <= to number of tracks on the radio
      // TODO: validate seconds_played_on_last_radio_track: radio has tracks on it
      // TODO: validate seconds_played_on_last_radio_track: is >= 0
      // TODO: validate seconds_played_on_last_radio_track: is <= the lenght of the last played track
        None
    }

    pub fn find(id: i32) -> Result<Option<Radio>, String> {
        let database_connection = POOL.get().expect("Failed to fetch a connection");

        match print(radios::table.find(id)).get_result(database_connection.deref()).optional() {
            Ok(radio) => Ok(radio),
            Err(error) => Err(format!("Error finding radio: {:?}", error))
        }
    }

    pub fn update<'a>(
        &'a self,
        user_id: Option<i32>,
        last_played_radio_track_number: Option<i32>,
        name: Option<&'a str>,
        seconds_played_on_last_radio_track: Option<i32>,
    ) -> Result<Radio, String> {
        if let Some(error) = Radio::validate(user_id, last_played_radio_track_number, name, seconds_played_on_last_radio_track) {
            return Err(format!("Error validating radio: {}", error))
        }

        let database_connection = POOL.get().expect("Failed to fetch a connection");
        let updated_radio = RadioUpdater {
            user_id,
            last_played_radio_track_number,
            name,
            seconds_played_on_last_radio_track,
            updated_at: Utc::now().naive_utc(),
        };
        let radio_update_query = diesel::update(radios::table.find(self.id)).set(&updated_radio);
        match print(radio_update_query).get_result(database_connection.deref()) {
            Ok(radio) => Ok(radio),
            Err(error) => Err(format!("Error updating radio: {:?}", error)),
        }
    }

    pub fn delete(id: i32) -> Result<Option<Radio>, String> {
        let database_connection = POOL.get().expect("Failed to fetch a connection");

        match print(diesel::delete(radios::table.find(id))).get_result(database_connection.deref()).optional() {
            Ok(radio) => Ok(radio),
            Err(error) => Err(format!("Error deleting radio: {:?}", error)),
        }
    }

    pub fn user(&self) -> Option<User> {
        let user_id = match self.user_id {
            Some(id) => id,
            None => return None,
        };
        match User::find(user_id) {
            Ok(optional_user) => optional_user,
            Err(error) => {
                println!("Error finding user: {:?}", error);
                return None
            }
        }
    }

    pub fn tracks(&self) -> Option<Vec<RadioTrack>> {
        let database_connection = POOL.get().expect("Failed to fetch a connection.");
        radio_tracks::table.filter(radio_tracks::radio_id.eq(self.id))
            .order(radio_tracks::track_order.asc())
            .get_results(database_connection.deref())
            .optional()
            .expect("Failed to fetch radio tracks")
    }
}

impl Deletable for Radio {
    fn delete(&self) -> Result<Option<Radio>, String> {
        match Radio::delete(self.id) {
            Ok(deleted) => Ok(deleted),
            Err(error) => Err(error),
        }
    }
}

impl<'a> NewRadio<'a> {
    pub fn save(&self) -> Result<Radio, diesel::result::Error> {
        let database_connection = POOL.get().expect("Failed to fetch a connection.");

        print(diesel::insert_into(radios::table).values(self)).get_result(database_connection.deref())
    }
}
