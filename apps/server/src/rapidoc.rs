use std::fmt::{Display, Formatter};

#[allow(dead_code)]
pub enum NavBarShowMethodAs {
    PlainText,
    ColoredText,
    ColoredBlock,
}

impl Display for NavBarShowMethodAs {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let val = match &self {
            NavBarShowMethodAs::PlainText => "as-plain-text",
            NavBarShowMethodAs::ColoredText => "as-colored-text",
            NavBarShowMethodAs::ColoredBlock => "as-colored-block",
        };

        write!(fmt, "{}", val)
    }
}

pub fn slot_nav_logo(src: &str, width: u32, height: u32) -> String {
    format!(
        r#"<img slot="nav-logo" src="{}" alt="Navbar Logo" style="max-width: {}px; max-height: {}px; align-self: center; padding-bottom: 16px"/>"#,
        src, width, height
    )
}
