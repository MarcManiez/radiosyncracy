use ::models::track::Track;

pub fn create_basic_track() -> Track {
    Track::create(None, "https://www.youtube.com/watch?v=-sRXOICYgHE", None).unwrap()
}
