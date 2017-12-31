use std::env;

pub const DEVELOPMENT: &'static str = "development";
pub const _PRODUCTION: &'static str = "production";
pub const TEST: &'static str = "test";

pub fn get() -> String {
    match env::var("ENV") {
        Ok(environment) => environment,
        Err(_) => String::from(DEVELOPMENT),
    }
}