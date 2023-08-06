use rocket::figment::providers::Serialized;
use rocket::{routes, Config};
use rocket_include_static_resources::static_resources_initializer;
use rocket_okapi::openapi_get_routes;

use crate::config::ServerConfig;

mod config;
mod routes;

#[rocket::launch]
async fn rocket() -> _ {
    let figment = Config::figment().merge(Serialized::defaults(ServerConfig::default()));
    rocket::custom(figment)
        .attach(static_resources_initializer!(
            "favicon" => "assets/favicon.ico"
        ))
        .mount("/", routes![routes::serve::favicon])
        .mount("/", openapi_get_routes![routes::index::index])
}
