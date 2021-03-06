extern crate router;

use ::controllers::*;
use self::router::Router;

pub fn get_routes() -> Router {
    let mut router = Router::new();
    router.post("/sessions", sessions_controller::create, "sessions#create");
    router.post("/tracks", tracks_controller::create, "tracks#create");
    router.delete("/tracks", tracks_controller::delete, "tracks#delete");
    router.get("/users", users_controller::find, "users#find");
    router.post("/users", users_controller::create, "users#create");
    router
}
