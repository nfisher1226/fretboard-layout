#![warn(clippy::all, clippy::pedantic)]
use crate::{Color, RGBA};

/// All of the configuration values which can be set in config.toml get stored
/// in this struct
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
    pub font: Option<String>,
}

impl Config {
    /// Creates a [Config] struct with default values
    pub fn default() -> Config {
        Config {
            border: 0.0,
            line_weight: 1.0,
            fretline_color: Color::Rgba(RGBA::white()),
            fretboard_color: Color::Rgba(RGBA::black()),
            centerline_color: Some(Color::Rgba(RGBA::blue())),
            font: Some(String::from("Sans Regular 12")),
        }
    }
}
