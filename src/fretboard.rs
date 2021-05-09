#![warn(clippy::all, clippy::pedantic)]
use crate::backend::{Factors, HexColor};
use crate::prefs::Config;
use crate::Specs;
use std::f64;
use std::str::FromStr;
use svg::node::element::{path::Data, Path};

/// Distance from bridge to fret along each side of the fretboard.
pub struct Lengths {
    pub length_bass: f64,
    pub length_treble: f64,
}

pub struct Point(pub f64, pub f64);

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Lengths {
    /// Plots the end of a fret, nut or bridge along the bass side of the scale
    fn get_point_bass(&self, factors: &Factors, config: &Config) -> Point {
        let x = (factors.x_ratio * self.length_bass) + config.border;
        let y = (factors.y_ratio * self.length_bass) + config.border;
        Point(x, y)
    }
    /// Plots the end of a fret, nut or bridge along the treble side of the scale
    fn get_point_treble(&self, factors: &Factors, specs: &Specs, config: &Config) -> Point {
        let x = factors.treble_offset + (factors.x_ratio * self.length_treble) + config.border;
        let y = specs.bridge - (factors.y_ratio * self.length_treble) + config.border;
        Point(x, y)
    }
    /// Returns a Point struct containing both ends of a fret, nut or bridge
    /// which will form a line
    pub fn get_fret_line(&self, factors: &Factors, specs: &Specs, config: &Config) -> Line {
        let start = self.get_point_bass(&factors, &config);
        let end = self.get_point_treble(&factors, &specs, &config);
        Line { start, end }
    }
}

impl Line {
    /// Returns an svg Path node representing a single fret
    pub fn draw_fret(&self, fret: u32, config: &Config) -> svg::node::element::Path {
        let id = if fret == 0 {
            "Nut".to_string()
        } else {
            format!("Fret {}", fret)
        };
        let hex = if let Ok(color) = gdk::RGBA::from_str(&config.fretline_color) {
            HexColor::from_rgba(color)
        } else {
            HexColor {
                color: String::from("#ffffff"),
                alpha: 1.0,
            }
        };
        let data = Data::new()
            .move_to((self.start.0, self.start.1))
            .line_to((self.end.0, self.end.1))
            .close();
        Path::new()
            .set("fill", "none")
            .set("stroke", hex.color)
            .set("stroke-opacity", hex.alpha)
            .set("stroke-width", config.line_weight)
            .set("id", id)
            .set("d", data)
    }
}
