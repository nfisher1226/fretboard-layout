#![warn(clippy::all, clippy::pedantic)]
use rgba_simple::{Color, RGBA};
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub enum FontWeight {
    Thin,
    Ultralight,
    Light,
    Semilight,
    Book,
    Normal,
    Medium,
    Semibold,
    Bold,
    Ultrabold,
    Heavy,
    Ultraheavy,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Font {
    pub family: String,
    pub weight: FontWeight,
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FontWeight {
    pub fn from_str(str: &str) -> Option<FontWeight> {
        match str {
            "Thin" | "thin" => Some(FontWeight::Thin),
            "Ultralight" | "ultralight" => Some(FontWeight::Ultralight),
            "Light" | "light" => Some(FontWeight::Light),
            "Semilight" | "semilight" => Some(FontWeight::Semilight),
            "Book" | "book" => Some(FontWeight::Book),
            "Normal" | "normal" => Some(FontWeight::Normal),
            "Medium" | "medium" => Some(FontWeight::Medium),
            "Semibold" | "semibold" => Some(FontWeight::Semibold),
            "Bold" | "bold" => Some(FontWeight::Bold),
            "Ultrabold" | "ultrabold" => Some(FontWeight::Ultrabold),
            "Heavy" | "heavy" => Some(FontWeight::Heavy),
            "Ultraheavy" | "ultraheavy" => Some(FontWeight::Ultraheavy),
            _ => None,
        }
    }
}

impl Font {
    /// Returns "Sans Normal"
    pub fn default() -> Font {
        Font {
            family: String::from("Sans"),
            weight: FontWeight::Normal,
        }
    }
}

/// All of the configuration values which can be set in config.toml get stored
/// in this struct
#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    /// The border which will appear around the rendering
    pub border: f64,
    /// The line weight for all of the elements in mm
    pub line_weight: f64,
    /// The color of the fret lines
    pub fretline_color: Color,
    /// The background color of the fretboard
    pub fretboard_color: Color,
    /// The color of the centerline
    pub centerline_color: Option<Color>,
    /// The font used for the specifications
    pub font: Option<Font>,
}

impl Config {
    /// Creates a [Config] struct with default values
    pub fn default() -> Config {
        Config {
            border: 10.0,
            line_weight: 1.0,
            fretline_color: Color::Rgba(RGBA::white()),
            fretboard_color: Color::Rgba(RGBA::black()),
            centerline_color: Some(Color::Rgba(RGBA::blue())),
            font: Some(Font::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let cfg = Config::default();
        assert_eq!(cfg.border, 10.0);
        assert_eq!(cfg.line_weight, 1.0);
        match cfg.fretline_color {
            Color::Rgba(color) => {
                assert_eq!(color.red, 1.0);
                assert_eq!(color.green, 1.0);
                assert_eq!(color.blue, 1.0);
            },
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn font_weight_from_str() {
        assert_eq!(None, FontWeight::from_str("foo"));
        assert_eq!(Some(FontWeight::Bold), FontWeight::from_str("bold"));
    }
}
