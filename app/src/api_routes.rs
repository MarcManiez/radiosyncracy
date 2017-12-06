extern crate router;

use ::controllers::*;
use self::router::Router;

pub fn get_routes() -> Router {
    let mut router = Router::new();
    router.get("/users", users_controller::get_all_users, "resource");
    router
}
