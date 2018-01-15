use ::models::radio::Radio;

pub fn create_basic_radio() -> Radio {
    Radio::create(None, "Hits from the '90s").unwrap()
}
