extern crate router;

use ::controllers::*;
use self::router::Router;

pub fn get_routes() -> Router {
    let mut router = Router::new();
    router.get("/users", users_controller::get_all_users, "resource");
    router.post("/users", users_controller::create, "resource");
    router
}
