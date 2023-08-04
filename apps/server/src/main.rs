use rocket::routes;
use rocket_include_static_resources::static_resources_initializer;

mod routes;

#[rocket::launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "assets/favicon.ico"
        ))
        .mount("/", routes![routes::serve::favicon, routes::index::index])
}
