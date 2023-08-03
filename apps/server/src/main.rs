use rocket::routes;
use rocket_include_static_resources::static_resources_initializer;

mod routes;

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "assets/favicon.ico"
        ))
        .mount("/", routes![routes::serve::favicon, routes::index::index]);

    Ok(rocket.into())
}
