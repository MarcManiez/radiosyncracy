extern crate router;

use ::controllers::*;
use self::router::Router;

pub fn get_routes() -> Router {
    let mut router = Router::new();
    router.post("/sessions", sessions_controller::create, "sessions#create");
    router.get("/users", users_controller::get_all_users, "users#get_all_users");
    router.post("/users", users_controller::create, "users#create");
    router
}
