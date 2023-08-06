use rocket::get;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct HelloWorld {
    message: &'static str,
}

#[openapi]
#[get("/")]
pub fn index() -> Json<HelloWorld> {
    Json(HelloWorld {
        message: "Hello, world!",
    })
}
