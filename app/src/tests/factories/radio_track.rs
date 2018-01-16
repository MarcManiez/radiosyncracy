use ::models::radio::Radio;
use ::models::radio_track::RadioTrack;
use ::models::track::Track;
use super::radio::create_basic_radio;
use super::track::create_basic_track;

pub fn create_radio_track() -> (RadioTrack, Radio, Track) {
    let radio = create_basic_radio();
    let track = create_basic_track();
    let radio_track = RadioTrack::create(Some(track.id), Some(radio.id), Some(1)).unwrap();
    let _ = radio.update(None, Some(1), None, Some(0));
    (radio_track, radio, track)
}
