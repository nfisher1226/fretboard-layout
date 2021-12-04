#![warn(clippy::all, clippy::pedantic)]
//! Fretboard_layout is a library for turning a set of specifications into a
//! complete template of a stringed musical instrument fretboard, such as a
//! guitar, banjo, or mandolin.
//! ## Usage
//!```rust
//!use fretboard_layout::{Config,Specs};
//!
//!fn main() {
//!    // the [Specs] struct constains the specifications used to generate the svg
//!    let mut specs = Specs::default();
//!    specs.set_multi(Some(615.0));
//!    specs.set_scale(675.0);
//!    // the (optional) [Config] struct fine tunes the visual representation
//!    let mut cfg = Config::default();
//!    cfg.set_line_weight(0.5);
//!    let svg = specs.create_document(Some(cfg));
//!}
//!```

pub mod config;
pub use config::{Config, Units};
pub mod layout;

use layout::Lengths;
use rgba_simple::{Color, RGBA};
use svg::node::element::{path::Data, Description, Group, Path};
use svg::Document;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Handedness {
    Right,
    Left,
}

#[derive(Debug, PartialEq)]
pub enum Variant {
    Monoscale,
    Multiscale(f64, Handedness),
}

impl Variant {
    fn default() -> Variant {
        Variant::Monoscale
    }

    fn value(&self) -> Option<f64> {
        match self {
            Variant::Monoscale => None,
            Variant::Multiscale(x, _) => Some(*x),
        }
    }

    fn handedness(&self) -> Option<Handedness> {
        match self {
            Variant::Monoscale => None,
            Variant::Multiscale(_, x) => Some(*x),
        }
    }
}

/// This struct contains the user data used to create the svg output file
pub struct Specs {
    /// Scale length. For multiscale designs this is the bass side scale length.
    pub scale: f64,
    /// Number of frets to render
    pub count: u32,
    /// Monoscale or Multiscale Right orLeft handed
    pub variant: Variant,
    /// The width of the fretboard at the nut.
    pub nut: f64,
    /// The string spacing at the bridge. Note that this is not the physical
    /// width of the bridge, but the distance perpendicular to the centerline
    /// between the outer two strings.
    pub bridge: f64,
    /// The fret that is perpendicular to the centerline.
    pub pfret: f64,
}

/// This struct contains multiplication factors used to convert the raw lengths
/// from bridge to fret into x,y coordinates. It also contains an offset distance
/// used to correctly orient the two scales in a multiscale design so that the
/// desired fret is perpendicular to the centerline.
pub struct Factors {
    pub x_ratio: f64,
    pub y_ratio: f64,
    pub treble_offset: f64,
}

impl Specs {
    /// Returns a default Specs struct
    pub fn default() -> Specs {
        Specs {
            scale: 655.0,
            count: 24,
            variant: Variant::default(),
            nut: 43.0,
            bridge: 56.0,
            pfret: 8.0,
        }
    }

    /// Returns a multiscale Specs struct
    pub fn multi() -> Specs {
        Specs {
            scale: 655.0,
            count: 24,
            variant: Variant::Multiscale(610.0, Handedness::Right),
            nut: 43.0,
            bridge: 56.0,
            pfret: 8.0,
        }
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    pub fn set_count(&mut self, count: u32) {
        self.count = count;
    }

    pub fn set_multi(&mut self, scale: Option<f64>) {
        match scale {
            Some(s) => {
                if let Some(hand) = self.variant.handedness() {
                    self.variant = Variant::Multiscale(s, hand);
                } else {
                    self.variant = Variant::Multiscale(s, Handedness::Right);
                };
            }
            None => self.variant = Variant::Monoscale,
        }
    }

    pub fn set_nut(&mut self, nut: f64) {
        self.nut = nut;
    }

    pub fn set_bridge(&mut self, bridge: f64) {
        self.bridge = bridge;
    }

    pub fn set_pfret(&mut self, pfret: f64) {
        self.pfret = pfret;
    }

    /// Returns the distance from bridge to nut on both sides of the fretboard
    fn get_nut(&self) -> Lengths {
        let length_treble = match self.variant {
            Variant::Multiscale(s, _) => s,
            Variant::Monoscale => self.scale,
        };
        Lengths {
            length_bass: self.scale,
            length_treble,
        }
    }

    /// Returns the length from bridge to fret for a given fret number, along
    /// both bass and treble sides of the board.
    pub fn get_fret_lengths(&self, fret: u32) -> Lengths {
        let factor = 2.0_f64.powf(f64::from(fret) / 12.0);
        let length_bass = self.scale / factor;
        let length_treble = match self.variant {
            Variant::Monoscale => length_bass,
            Variant::Multiscale(s, _) => s / factor,
        };
        Lengths {
            length_bass,
            length_treble,
        }
    }

    /// Returns a vector containing the lengths from bridge to fret for all of
    /// the frets to be rendered.
    pub fn get_all_lengths(&self) -> Vec<Lengths> {
        let mut fretboard: Vec<Lengths> = Vec::new();
        let nut = self.get_nut();
        fretboard.push(nut);
        for n in 1..self.count + 2 {
            let fret = self.get_fret_lengths(n);
            fretboard.push(fret);
        }
        fretboard
    }

    /// Uses trigonometry to place the fret ends, based on visualizing their
    /// locations as a triangle where the hypotenuse is the string, and the
    /// opposite is the distance from the bridge parallel to the centerline.
    fn get_factors(&self) -> Factors {
        let height = (self.bridge - self.nut) / 2.0;
        let y_ratio = height / self.scale;
        let x_ratio = y_ratio.acos().sin();
        let factor = 2.0_f64.powf(self.pfret / 12.0);
        let length_bass = self.scale / factor;
        let length_treble = match self.variant {
            Variant::Monoscale => length_bass,
            Variant::Multiscale(s, _) => s / factor,
        };
        let bass_pfret = x_ratio * length_bass;
        let treble_pfret = x_ratio * length_treble;
        let treble_offset = bass_pfret - treble_pfret;
        Factors {
            x_ratio,
            y_ratio,
            treble_offset,
        }
    }

    /// Embeds a text description into the svg
    fn create_description(&self) -> svg::node::element::Description {
        Description::new()
            .set("Scale", self.scale)
            .set(
                "Multiscale",
                match self.variant {
                    Variant::Monoscale => false,
                    Variant::Multiscale(_, _) => true,
                },
            )
            .set(
                "ScaleTreble",
                match self.variant {
                    Variant::Monoscale => self.scale,
                    Variant::Multiscale(s, _) => s,
                },
            )
            .set("PerpendicularFret", self.pfret)
            .set("BridgeSpacing", self.bridge - 6.0)
            .set("NutWidth", self.nut)
    }

    /// Prints the specs used in the rendered image
    fn print_data(&self, config: &Config) -> svg::node::element::Text {
        let units = match config.units {
            Units::Metric => String::from("mm"),
            Units::Imperial => String::from("in"),
        };
        let mut line = match self.variant {
            Variant::Monoscale => format!("Scale: {:.2}{} |", self.scale, &units),
            Variant::Multiscale(s, _) => format!(
                "ScaleBass: {:.2}{} | ScaleTreble: {:.2}{} | PerpendicularFret: {:.1} |",
                self.scale, &units, s, &units, self.pfret
            ),
        };
        let font_family = match &config.font {
            Some(font) => String::from(&font.family),
            None => String::from("Sans"),
        };
        let font_weight = match &config.font {
            Some(font) => font.weight.to_string(),
            None => String::from("Regular"),
        };
        line = format!("{} NutWidth: {:.2}{} |", line, self.nut, &units);
        line = format!("{} BridgeSpacing: {:.2}{}", line, self.bridge - 6.0, &units);
        svg::node::element::Text::new()
            .set("x", config.border)
            .set("y", (config.border * 1.7) + self.bridge)
            .set("font-family", font_family)
            .set("font-weight", font_weight)
            .set("font-size", "5px")
            .set("id", "Specifications")
            .add(svg::node::Text::new(line))
    }

    /// Adds the centerline to the svg data
    fn draw_centerline(&self, config: &Config) -> svg::node::element::Path {
        let start_x = config.border;
        let start_y = (self.bridge / 2.0) + config.border;
        let end_x = config.border + self.scale;
        let end_y = (self.bridge / 2.0) + config.border;
        let hex = config
            .centerline_color
            .as_ref()
            .unwrap_or(&Color::Rgba(RGBA::blue()))
            .to_hex();
        let data = Data::new()
            .move_to((start_x, start_y))
            .line_to((end_x, end_y))
            .close();
        Path::new()
            .set("fill", "none")
            .set("stroke", hex.color.as_str())
            .set("stroke-opacity", hex.alpha)
            .set("stroke-dasharray", "4.0, 8.0")
            .set("stroke-dashoffset", "0")
            .set("stroke-width", config.line_weight)
            .set("id", "Centerline")
            .set("d", data)
    }

    /// adds the bridge as a line between the outer strings
    fn draw_bridge(&self, factors: &Factors, config: &Config) -> svg::node::element::Path {
        let start_x = match self.variant {
            Variant::Monoscale | Variant::Multiscale(_, Handedness::Right) => config.border,
            Variant::Multiscale(_, Handedness::Left) => config.border + factors.treble_offset,
        };
        let start_y = config.border;
        let end_x = match self.variant {
            Variant::Monoscale | Variant::Multiscale(_, Handedness::Right) => {
                config.border + factors.treble_offset
            }
            Variant::Multiscale(_, Handedness::Left) => config.border,
        };
        let end_y = config.border + self.bridge;
        let data = Data::new()
            .move_to((start_x, start_y))
            .line_to((end_x, end_y))
            .close();
        Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", config.line_weight)
            .set("id", "Bridge")
            .set("d", data)
    }

    /// Draws the outline of the fretboard
    fn draw_fretboard(
        &self,
        fretboard: &[Lengths],
        factors: &Factors,
        config: &Config,
    ) -> svg::node::element::Path {
        let nut = fretboard[0_usize].get_fret_line(&factors, &self, &config);
        let end = fretboard[self.count as usize + 1].get_fret_line(&factors, &self, &config);
        let hex = config.fretboard_color.to_hex();
        let data = Data::new()
            .move_to((nut.start.0, nut.start.1))
            .line_to((nut.end.0, nut.end.1))
            .line_to((end.end.0, end.end.1))
            .line_to((end.start.0, end.start.1))
            .line_to((nut.start.0, nut.start.1))
            .close();
        Path::new()
            .set("fill", hex.color)
            .set("fill-opacity", hex.alpha)
            .set("stroke", "none")
            .set("id", "Fretboard")
            .set("d", data)
    }

    /// Iterates through each fret, returning a group of svg Paths
    fn draw_frets(
        &self,
        fretboard: &[Lengths],
        factors: &Factors,
        config: &Config,
    ) -> svg::node::element::Group {
        let mut frets = Group::new().set("id", "Frets");
        for fret in 0..=self.count {
            let line = fretboard[fret as usize].get_fret_line(&factors, &self, &config);
            frets = frets.add(line.draw_fret(fret, &config));
        }
        frets
    }

    ///Returns the complete svg Document
    ///# Example
    ///
    ///```rust
    ///use fretboard_layout::{Config, Specs};
    ///
    ///fn run() {
    ///    let specs = Specs::default();
    ///    let doc = specs.create_document(Some(Config::default()));
    ///}
    ///```
    pub fn create_document(&self, conf: Option<Config>) -> svg::Document {
        let config = conf.unwrap_or_else(Config::default);
        let lengths: Vec<Lengths> = self.get_all_lengths();
        let factors = &self.get_factors();
        let width = (config.border * 2.0) + self.scale;
        let widthmm = format!("{}mm", width);
        let height = (config.border * 2.0) + self.bridge;
        let heightmm = format!("{}mm", height);
        let document = Document::new()
            .set("width", widthmm)
            .set("height", heightmm)
            .set("preserveAspectRatio", "xMidYMid meet")
            .set("viewBox", (0, 0, width, height))
            .add(self.create_description())
            .add(self.draw_fretboard(&lengths, &factors, &config))
            .add(self.draw_bridge(&factors, &config))
            .add(self.draw_frets(&lengths, &factors, &config));
        if config.font.is_some() {
            if config.centerline_color.is_some() {
                document
                    .add(self.print_data(&config))
                    .add(self.draw_centerline(&config))
            } else {
                document.add(self.print_data(&config))
            }
        } else if config.centerline_color.is_some() {
            document.add(self.draw_centerline(&config))
        } else {
            document
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_default() {
        let var = Variant::default();
        assert_eq!(Variant::Monoscale, var);
    }

    #[test]
    fn variant_value() {
        let var = Variant::Multiscale(23.5, Handedness::Right);
        let val = var.value();
        assert_eq!(val.unwrap(), 23.5);
        let hand = var.handedness();
        assert_eq!(hand.unwrap(), Handedness::Right);
    }

    #[test]
    fn factors_default() {
        let factors = Specs::default().get_factors();
        assert_eq!(factors.x_ratio, 0.9999507592328689);
        assert_eq!(factors.y_ratio, 0.009923664122137405);
        assert_eq!(factors.treble_offset, 0.0);
    }

    #[test]
    fn factors_multi() {
        let factors = Specs::multi().get_factors();
        assert_eq!(factors.x_ratio, 0.9999507592328689);
        assert_eq!(factors.y_ratio, 0.009923664122137405);
        assert_eq!(factors.treble_offset, 28.346827734356623);
    }
}
