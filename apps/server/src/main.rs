use okapi::openapi3::{Info, OpenApi};
use rocket::figment::providers::Serialized;
use rocket::Config;
use rocket::{get, routes};
use rocket_include_static_resources::{
    cached_static_response_handler, static_resources_initializer,
};
use rocket_okapi::rapidoc::{
    make_rapidoc, DefaultSchemaTab, FontSize, GeneralConfig, HideShowConfig, Layout, LayoutConfig,
    NavConfig, NavItemSpacing, RapiDocConfig, RenderStyle, SchemaStyle, Theme, UiConfig,
};
use rocket_okapi::settings::{OpenApiSettings, UrlObject};
use rocket_okapi::{hash_map, mount_endpoints_and_merged_docs, openapi_get_routes_spec};

use crate::config::ServerConfig;
use crate::rapidoc::{slot_nav_logo, NavBarShowMethodAs};

mod config;
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
    // Rocket
    let figment = Config::figment().merge(Serialized::defaults(ServerConfig::default()));
    let mut rocket = rocket::custom(figment);

    // OpenAPI
    let settings = OpenApiSettings::default();
    let specs = OpenApi {
        openapi: OpenApi::default_version(),
        info: Info {
            title: "NosAPI".to_owned(),
            description: Some("".to_owned()),
            ..Default::default()
        },
        ..Default::default()
    };

    mount_endpoints_and_merged_docs! {
        rocket, "/v1".to_owned(), settings,
        "" => (vec![], specs),
        "" => openapi_get_routes_spec![settings: routes::hello::hello],
    };

    rocket
        .attach(static_resources_initializer!(
            "favicon" => "assets/favicon.ico",
            "favicon-16" => "assets/favicon-16x16.png",
            "favicon-32" => "assets/favicon-32x32.png",
            "logo" => "assets/rapidoc/logo.png"
        ))
        .mount("/", routes![favicon])
        .mount("/assets/", routes![logo])
        .mount(
            "/",
            make_rapidoc(&RapiDocConfig {
                title: Some("NosAPI | Docs".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("V1", "/v1/openapi.json")],
                    ..Default::default()
                },
                ui: UiConfig {
                    theme: Theme::Light,
                    primary_color: "#FF9900".to_owned(),
                    regular_font: "Open Sans".to_owned(),
                    font_size: FontSize::Default,
                    ..Default::default()
                },
                nav: NavConfig {
                    use_path_in_nav_bar: false,
                    nav_item_spacing: NavItemSpacing::Relaxed,
                    ..Default::default()
                },
                layout: LayoutConfig {
                    layout: Layout::Row,
                    render_style: RenderStyle::Focused,
                    schema_style: SchemaStyle::Table,
                    schema_expand_level: 2,
                    schema_description_expanded: true,
                    default_schema_tab: DefaultSchemaTab::Model,
                    response_area_height: "300px".to_owned(),
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    show_info: false,
                    show_header: false,
                    show_components: false,
                    allow_authentication: false,
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                custom_template_tags: hash_map! {
                    "NAV_LOGO".to_owned() => slot_nav_logo("/assets/logo.png", 200, 200),
                    "SHOW_METHOD_IN_NAV_BAR".to_owned() => NavBarShowMethodAs::ColoredText.to_string()
                },
                custom_html: Some(include_str!("../assets/rapidoc/index.html").to_owned()),
                ..Default::default()
            }),
        )
}
