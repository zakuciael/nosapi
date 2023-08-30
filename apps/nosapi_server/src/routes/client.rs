use std::io::Read;

use rocket::get;
use rocket::http::hyper::body::Buf;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, JsonSchema};
use serde::{Deserialize, Serialize};

use nosapi_data::client::{get_client_version, ClientVersion};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct HelloWorld {
  message: &'static str,
}

#[openapi]
#[get("/client/version")]
/// Print Hello, World!
pub async fn get_version() -> Json<ClientVersion> {
  // TODO: Implement error handling
  // TODO: Concurrently call file requests
  // TODO: Dynamically resolve the file url
  // TODO: Create util function to read the whole file to the memory
  // TODO: Add caching for the responses
  // TODO: Connect a database that stores the resolved data and update it periodically / or on-demand
  // TODO: get_client_version might accept a type that implements Read trait instead of a specific type

  let mut client_x = vec![];
  let mut client_gl = vec![];

  reqwest::get("http://patches.gameforge.com/38/382f76038e28d3e427921101f612e1a2178ab87b/382f76038e28d3e427921101f612e1a2178ab87b-3655216")
        .await.unwrap()
        .bytes()
        .await.unwrap()
        .reader().read_to_end(&mut client_x).unwrap();

  reqwest::get("http://patches.gameforge.com/53/532c0582003e6d368409cbfbb954ac9029d95c96/532c0582003e6d368409cbfbb954ac9029d95c96-3595312")
        .await.unwrap()
        .bytes()
        .await.unwrap()
        .reader().read_to_end(&mut client_gl).unwrap();

  Json(
    get_client_version(&mut client_x, &mut client_gl)
      .unwrap()
      .unwrap(),
  )
}
