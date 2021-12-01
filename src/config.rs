#![warn(clippy::all, clippy::pedantic)]
use rgba_simple::{Color, RGBA};
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub enum Units {
    Metric,
    Imperial,
}

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

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Units {
    /// Returns Units::Metric
    pub fn default() -> Units {
        Units::Metric
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FontWeight {
    pub fn default() -> FontWeight {
        FontWeight::Normal
    }

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
            weight: FontWeight::default(),
        }
    }

    pub fn set_family(&mut self, family: String) {
        self.family = family;
    }

    pub fn set_weight(&mut self, weight: FontWeight) {
        self.weight = weight;
    }
}

/// All of the configuration values which can be set in config.toml get stored
/// in this struct
#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    /// Whether to use Millimeters (mm) or Inches (in) when displaying lengths
    pub units: Units,
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
            units: Units::default(),
            border: 10.0,
            line_weight: 1.0,
            fretline_color: Color::Rgba(RGBA::white()),
            fretboard_color: Color::Rgba(RGBA::black()),
            centerline_color: Some(Color::Rgba(RGBA::blue())),
            font: Some(Font::default()),
        }
    }

    pub fn set_units(&mut self, units: Units) {
        self.units = units;
    }

    pub fn set_border(&mut self, border: f64) {
        self.border = border;
    }

    pub fn set_line_weight(&mut self, weight: f64) {
        self.line_weight = weight;
    }

    pub fn set_fretline_color(&mut self, color: Color) {
        self.fretline_color = color;
    }

    pub fn set_fretboard_color(&mut self, color: Color) {
        self.fretboard_color = color;
    }

    pub fn set_centerline_color(&mut self, color: Option<Color>) {
        self.centerline_color = color;
    }

    pub fn set_font(&mut self, font: Option<Font>) {
        self.font = font;
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
    fn change_cfg() {
        let mut cfg = Config::default();
        cfg.set_border(5.0);
        cfg.set_font(None);
        assert_eq!(cfg.border, 5.0);
        assert!(cfg.font.is_none());
    }

    #[test]
    fn font_weight_from_str() {
        assert_eq!(None, FontWeight::from_str("foo"));
        assert_eq!(Some(FontWeight::Bold), FontWeight::from_str("bold"));
    }
}
