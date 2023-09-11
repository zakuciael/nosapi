use okapi::openapi3::{Info, OpenApi};

pub fn make_openapi_specs() -> OpenApi {
  OpenApi {
    openapi: OpenApi::default_version(),
    info: Info {
      title: "NosAPI".to_owned(),
      description: Some("".to_owned()),
      ..Default::default()
    },
    ..Default::default()
  }
}
