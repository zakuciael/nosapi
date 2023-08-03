use rocket::{get, State};
use rocket_include_static_resources::{
    CacheResponse, EtagIfNoneMatch, StaticContextManager, StaticResponse,
};

#[get("/favicon.ico")]
pub fn favicon(
    static_resources: &State<StaticContextManager>,
    etag_if_none_match: EtagIfNoneMatch,
) -> CacheResponse<StaticResponse> {
    let responder = static_resources.build(&etag_if_none_match, "favicon");
    CacheResponse::public_only_release(responder, 84600, false)
}
