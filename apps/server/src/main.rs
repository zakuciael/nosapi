use rocket::figment::providers::Serialized;
use rocket::Config;
use rocket::{get, routes};
use rocket_include_static_resources::{
    cached_static_response_handler, static_resources_initializer,
};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi_get_routes_spec};

use crate::config::ServerConfig;
use crate::openapi::make_openapi_specs;
use crate::rapidoc::make_rapidoc;

mod config;
mod openapi;
mod rapidoc;
mod routes;

cached_static_response_handler! {
    84600;
    "/favicon.ico" => favicon => "favicon",
    "/favicon-16x16.png" => favicon_16 => "favicon-16",
    "/favicon-32x32.png" => favicon_32 => "favicon-32",
    "/logo.png" => logo => "logo"
}

#[rocket::launch]
async fn rocket() -> _ {
    let settings = OpenApiSettings::default();
    let figment = Config::figment().merge(Serialized::defaults(ServerConfig::default()));
    let mut rocket = rocket::custom(figment);

    mount_endpoints_and_merged_docs! {
        rocket, "/v1".to_owned(), settings,
        "" => (vec![], make_openapi_specs()),
        "" => openapi_get_routes_spec![settings: routes::hello::hello],
    }

    rocket
        .attach(static_resources_initializer!(
            "favicon" => "assets/favicon.ico",
            "favicon-16" => "assets/favicon-16x16.png",
            "favicon-32" => "assets/favicon-32x32.png",
            "logo" => "assets/rapidoc/logo.png"
        ))
        .mount("/", routes![favicon])
        .mount("/docs/assets/", routes![logo])
        .mount("/docs/", make_rapidoc())
}
