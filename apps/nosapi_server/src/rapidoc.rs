use rocket::Route;
use rocket_okapi::hash_map;
use rocket_okapi::rapidoc::{
  DefaultSchemaTab, FontSize, GeneralConfig, HideShowConfig, Layout, LayoutConfig, NavConfig,
  NavItemSpacing, RapiDocConfig, RenderStyle, SchemaStyle, Theme, UiConfig,
};
use rocket_okapi::settings::UrlObject;

fn slot_nav_logo(src: &str, width: u32, height: u32) -> String {
  format!(
    r#"<img slot="nav-logo" src="{}" alt="Navbar Logo" style="max-width: {}px; max-height: {}px; align-self: center; padding-bottom: 16px"/>"#,
    src, width, height
  )
}

pub fn make_rapidoc() -> impl Into<Vec<Route>> {
  rocket_okapi::rapidoc::make_rapidoc(&RapiDocConfig {
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
        "NAV_LOGO".to_owned() => slot_nav_logo("/docs/assets/logo.png", 200, 200),
        "SHOW_METHOD_IN_NAV_BAR".to_owned() => "as-colored-text".to_owned()
    },
    custom_html: Some(include_str!("../assets/rapidoc/index.html").to_owned()),
    ..Default::default()
  })
}
