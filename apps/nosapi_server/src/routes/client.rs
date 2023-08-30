use std::io::Read;

use rocket::get;
use rocket::http::hyper::body::Buf;
use rocket::serde::json::Json;
use rocket::tokio::join;
use rocket_okapi::openapi;

use nosapi_data::client::{get_client_version, ClientVersion};

async fn download(url: &str) -> Vec<u8> {
  // TODO: Implement error handling
  let mut output = vec![];
  reqwest::get(url)
    .await
    .unwrap()
    .bytes()
    .await
    .unwrap()
    .reader()
    .read_to_end(&mut output)
    .unwrap();

  output
}

#[openapi]
#[get("/client/version")]
/// Get the latest client version information
pub async fn get_version() -> Json<ClientVersion> {
  // TODO: Implement error handling
  // TODO: Dynamically resolve the file url
  // TODO: Create util function to read the whole file to the memory
  // TODO: Add caching for the responses
  // TODO: Connect a database that stores the resolved data and update it periodically / or on-demand

  let (client_x, client_gl) = join!(
    download("http://patches.gameforge.com/22/22546be2bef67c2d0ddebbd0d2648cf18a5e525d/22546be2bef67c2d0ddebbd0d2648cf18a5e525d-3655216"),
    download("http://patches.gameforge.com/e3/e3fbed679ebadcad2571d4947695379d7d58546b/e3fbed679ebadcad2571d4947695379d7d58546b-3595824")
  );

  Json(get_client_version(&client_x, &client_gl).unwrap().unwrap())
}
