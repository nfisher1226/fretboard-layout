#![warn(clippy::all, clippy::pedantic)]

pub mod font;

use {
    crate::{PrimaryColor, RGBA},
    font::Font,
    serde::{Deserialize, Serialize},
    std::{error::Error, fmt, str::FromStr},
};

/// Whether to use Metric (millimeters) or Imperial (inches) measurements
#[derive(Clone, Copy, Deserialize, Debug, Eq, PartialEq, Serialize)]
pub enum Units {
    /// Output measurements are given in *millimeters*
    Metric,
    /// Output measurements are given in *inches*
    Imperial,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParseUnitsError;

impl fmt::Display for ParseUnitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing units from string")
    }
}

impl Error for ParseUnitsError {}

impl FromStr for Units {
    type Err = ParseUnitsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "metric" | "Metric" => Ok(Self::Metric),
            "imperial" | "Imperial" => Ok(Self::Imperial),
            _ => Err(ParseUnitsError),
        }
    }
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Metric => "metric",
                Self::Imperial => "imperial",
            }
        )
    }
}

impl Default for Units {
    /// Returns `Units::Metric`
    fn default() -> Self {
        Self::Metric
    }
}

/// All of the configuration values which can be set in config.toml get stored
/// in this struct
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Config {
    /// Whether to use Millimeters (mm) or Inches (in) when displaying lengths
    pub units: Units,
    /// The border which will appear around the rendering
    pub border: f64,
    /// The line weight for all of the elements in mm
    pub line_weight: f64,
    /// The color of the fret lines
    pub fretline_color: RGBA<u8>,
    /// The background color of the fretboard
    pub fretboard_color: RGBA<u8>,
    /// The color of the centerline
    pub centerline_color: Option<RGBA<u8>>,
    /// The font used for the specifications
    pub font: Option<Font>,
}

impl Default for Config {
    /// Creates a [Config] struct with default values
    fn default() -> Self {
        Self {
            units: Units::default(),
            border: 10.0,
            line_weight: 1.0,
            fretline_color: PrimaryColor::White.into(),
            fretboard_color: PrimaryColor::Black.into(),
            centerline_color: Some(PrimaryColor::Blue.into()),
            font: Some(Font::default()),
        }
    }
}

impl Config {
    #[allow(clippy::must_use_candidate)]
    pub fn units(&self) -> Units {
        self.units
    }

    pub fn set_units(&mut self, units: Units) {
        self.units = units;
    }

    #[allow(clippy::must_use_candidate)]
    pub fn border(&self) -> f64 {
        self.border
    }

    pub fn set_border(&mut self, border: f64) {
        self.border = border;
    }

    #[allow(clippy::must_use_candidate)]
    pub fn line_weight(&self) -> f64 {
        self.line_weight
    }

    pub fn set_line_weight(&mut self, weight: f64) {
        self.line_weight = weight;
    }

    #[allow(clippy::must_use_candidate)]
    pub fn fretline_color(&self) -> RGBA<u8> {
        self.fretline_color
    }

    pub fn set_fretline_color(&mut self, color: RGBA<u8>) {
        self.fretline_color = color;
    }

    #[allow(clippy::must_use_candidate)]
    pub fn fretboard_color(&self) -> RGBA<u8> {
        self.fretboard_color
    }

    pub fn set_fretboard_color(&mut self, color: RGBA<u8>) {
        self.fretboard_color = color;
    }

    #[allow(clippy::must_use_candidate)]
    pub fn centerline_color(&self) -> Option<RGBA<u8>> {
        self.centerline_color.as_ref().copied()
    }

    pub fn set_centerline_color(&mut self, color: Option<RGBA<u8>>) {
        self.centerline_color = color;
    }

    #[allow(clippy::must_use_candidate)]
    pub fn font(&self) -> Option<Font> {
        self.font.as_ref().cloned()
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
        assert_eq!(cfg.fretline_color.red, 255);
        assert_eq!(cfg.fretline_color.green, 255);
        assert_eq!(cfg.fretline_color.blue, 255);
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
        assert_eq!(Err(ParseFontError), Weight::from_str("foo"));
        assert_eq!(Ok(Weight::Bold), Weight::from_str("Weight::bold"));
    }
}
