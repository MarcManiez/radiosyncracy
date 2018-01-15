use chrono::prelude::*;
use diesel;
use diesel::{FindDsl, LoadDsl, OptionalExtension};
use regex::Regex;

use std::ops::Deref;

use ::connection::POOL;
use ::schema::radios;
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
