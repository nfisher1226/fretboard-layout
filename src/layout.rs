#![warn(clippy::all, clippy::pedantic)]
use crate::{Config, Factors, Handedness, Specs, Variant};
use std::f64;
use svg::node::element::{path::Data, Path};

/// Distance from bridge to fret along each side of the fretboard.
pub struct Lengths {
    pub length_bass: f64,
    pub length_treble: f64,
}

/// A 2-dimensional representation of a point
pub struct Point(pub f64, pub f64);

/// 2 Points which form a line
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Lengths {
    /// Plots the end of a fret, nut or bridge along the bass side of the scale
    fn get_point_bass(&self, factors: &Factors, specs: &Specs, config: &Config) -> Point {
        let x = (factors.x_ratio * self.length_bass) + config.border;
        let hand = specs.variant.handedness();
        let opposite = factors.y_ratio * self.length_bass;
        let y = match hand {
            Some(Handedness::Left) => specs.bridge - opposite + config.border,
            _ => opposite + config.border,
        };
        Point(x, y)
    }
    /// Plots the end of a fret, nut or bridge along the treble side of the scale
    fn get_point_treble(&self, factors: &Factors, specs: &Specs, config: &Config) -> Point {
        let x = factors.treble_offset + (factors.x_ratio * self.length_treble) + config.border;
        let hand = specs.variant.handedness();
        let opposite = factors.y_ratio * self.length_treble;
        let y = match hand {
            Some(Handedness::Left) => specs.bridge - opposite + config.border,
            _ => opposite + config.border,
        };
        Point(x, y)
    }
    /// Returns a Point struct containing both ends of a fret, nut or bridge
    /// which will form a line
    pub fn get_fret_line(&self, factors: &Factors, specs: &Specs, config: &Config) -> Line {
        let start = self.get_point_bass(&factors, specs, &config);
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
        let hex = config.fretline_color.to_hex();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lengths() {
        let specs = Specs::default();
        let lengths = specs.get_fret_lengths(12);
        assert_eq!(lengths.length_bass, 327.5);
        assert_eq!(lengths.length_treble, lengths.length_treble);
        let lengths = specs.get_fret_lengths(24);
        assert_eq!(lengths.length_bass, 163.75);
        assert_eq!(lengths.length_bass, lengths.length_treble);
    }
}
