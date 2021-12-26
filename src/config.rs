#![warn(clippy::all, clippy::pedantic)]
use rgba_simple::{Color, Primary, RGBA};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Whether to use Metric (millimeters) or Imperrial (inches) measurements
#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub enum Units {
    /// Output measurements are given in *millimeters*
    Metric,
    /// Output measurements are given in *inches*
    Imperial,
}

/// The weight, or style, of the font
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

/// The font used to print the description in the output file
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Font {
    /// The *family* , eg *Sans* or *ComicSans*
    pub family: String,
    /// The *weight* or *style* of the given font
    pub weight: FontWeight,
}

/// Error returned if unable to parse a font from a given `str`
#[derive(Debug, PartialEq)]
pub struct ParseFontError;

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Units {
    /// Returns `Units::Metric`
    fn default() -> Self {
        Units::Metric
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

impl FromStr for FontWeight {
    type Err = ParseFontError;

    #[allow(clippy::must_use_candidate)]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Style::Thin" | "Style::thin" => Ok(FontWeight::Thin),
            "Style::Ultralight" | "Style::ultralight" => Ok(FontWeight::Ultralight),
            "Style::Light" | "Style::light" => Ok(FontWeight::Light),
            "Style::Semilight" | "Style::semilight" => Ok(FontWeight::Semilight),
            "Style::Book" | "Style::book" => Ok(FontWeight::Book),
            "Style::Normal" | "Style::normal" => Ok(FontWeight::Normal),
            "Style::Medium" | "Style::medium" => Ok(FontWeight::Medium),
            "Style::Semibold" | "Style::semibold" => Ok(FontWeight::Semibold),
            "Style::Bold" | "Style::bold" => Ok(FontWeight::Bold),
            "Style::Ultrabold" | "Style::ultrabold" => Ok(FontWeight::Ultrabold),
            "Style::Heavy" | "Style::heavy" => Ok(FontWeight::Heavy),
            "Style::Ultraheavy" | "Style::ultraheavy" => Ok(FontWeight::Ultraheavy),
            _ => Err(ParseFontError),
        }
    }
}

impl Default for Font {
    /// Returns "Sans Normal"
    fn default() -> Font {
        Font {
            family: String::from("Sans"),
            weight: FontWeight::default(),
        }
    }
}

impl fmt::Display for ParseFontError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Font {
    /// Set the *family* of the font
    pub fn set_family(&mut self, family: String) {
        self.family = family;
    }

    /// Set the *weight* or *style* of the font
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

impl Default for Config {
    /// Creates a [Config] struct with default values
    fn default() -> Config {
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
}

impl Config {
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
            }
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
        assert_eq!(Err(ParseFontError), FontWeight::from_str("foo"));
        assert_eq!(Ok(FontWeight::Bold), FontWeight::from_str("Style::bold"));
    }
}
