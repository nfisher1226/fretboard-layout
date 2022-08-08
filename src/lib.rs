#![warn(clippy::all, clippy::pedantic)]
#![doc = include_str!("../README.md")]
mod config;
pub use {
    config::{Config, Font, FontWeight, Units},
    rgba_simple::*,
};

use {
    rayon::prelude::*,
    serde::{Deserialize, Serialize},
    std::f64,
    svg::{
        node::element::{path::Data, Description, Group, Path},
        Document,
    },
    PrimaryColor::Blue,
};

/// Whether the output represents a right handed or left handed neck style
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Handedness {
    Right,
    Left,
}

impl Default for Handedness {
    fn default() -> Self {
        Self::Right
    }
}

/// Whether to output a traditional `Monoscale` style neck with the same scale
/// across it's entire width, or a modern `Multiscale` neck, with a shorter scale
/// along the treble side, also known as *fan fret*.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Variant {
    /// A traditional fretbaord where the same scale length is used all of the
    /// way across the fretboard.
    Monoscale,
    /// A modern style of neck where there is a longer scale length along the
    /// bass side of the neck and a shorter scale along the treble side of the
    /// neck, allowing for more natural string tension, greater flexibility in
    /// tuning, and better ergonomics.
    Multiscale(
        /// The scale length along the treble side of the neck
        f64,
        /// Right or left handed output
        Handedness,
    ),
}

impl Default for Variant {
    fn default() -> Self {
        Self::Monoscale
    }
}

impl Variant {
    fn multi() -> Self {
        Self::Multiscale(610.0, Handedness::default())
    }

    /// Return the treble side scale length if the neck is `Multiscale`, or else
    /// `None`
    #[allow(clippy::must_use_candidate)]
    pub fn scale(&self) -> Option<f64> {
        match self {
            Self::Monoscale => None,
            Self::Multiscale(x, _) => Some(*x),
        }
    }

    /// Returns whether the resulting neck is right or left handed, or `None` if
    /// the neck is `Monoscale`
    #[allow(clippy::must_use_candidate)]
    pub fn handedness(&self) -> Option<Handedness> {
        match self {
            Self::Monoscale => None,
            Self::Multiscale(_, x) => Some(*x),
        }
    }
}

// This struct contains multiplication factors used to convert the raw lengths
// from bridge to fret into x,y coordinates. It also contains an offset distance
// used to correctly orient the two scales in a multiscale design so that the
// desired fret is perpendicular to the centerline.
struct Factors {
    x_ratio: f64,
    y_ratio: f64,
    // How far forward the treble side of the bridge should start with respect
    // to the trable side
    treble_offset: f64,
}

impl Default for Factors {
    fn default() -> Self {
        Self::init(655.0, &Variant::default(), 43.0, 56.0, 8.0)
    }
}

impl Factors {
    /// Uses trigonometry to place the fret ends, based on visualizing their
    /// locations as a triangle where the hypotenuse is the string, and the
    /// opposite is the distance from the bridge parallel to the centerline.
    fn init(scale: f64, variant: &Variant, nut: f64, bridge: f64, pfret: f64) -> Self {
        let height = (bridge - nut) / 2.0;
        let y_ratio = height / scale;
        let x_ratio = y_ratio.acos().sin();
        let factor = 2.0_f64.powf(pfret / 12.0);
        let length_bass = scale / factor;
        let length_treble = match variant {
            Variant::Monoscale => length_bass,
            Variant::Multiscale(s, _) => s / factor,
        };
        let bass_pfret = x_ratio * length_bass;
        let treble_pfret = x_ratio * length_treble;
        let treble_offset = bass_pfret - treble_pfret;
        Self {
            x_ratio,
            y_ratio,
            treble_offset,
        }
    }
}

/// Distance from bridge to fret along each side of the fretboard.
struct Lengths {
    length_bass: f64,
    length_treble: f64,
}

/// A 2-dimensional representation of a point
struct Point(pub f64, pub f64);

/// 2 Points which form a line
struct Line {
    start: Point,
    end: Point,
}

impl Lengths {
    /// Plots the end of a fret, nut or bridge along the bass side of the scale
    fn get_point_bass(&self, specs: &Specs, config: &Config) -> Point {
        let hand = specs.variant.handedness();
        let x = match hand {
            Some(Handedness::Left) => {
                specs.scale - (specs.factors.x_ratio * self.length_bass) + config.border
            }
            _ => (specs.factors.x_ratio * self.length_bass) + config.border,
        };
        let opposite = specs.factors.y_ratio * self.length_bass;
        let y = opposite + config.border;
        Point(x, y)
    }
    /// Plots the end of a fret, nut or bridge along the treble side of the scale
    fn get_point_treble(&self, specs: &Specs, config: &Config) -> Point {
        let hand = specs.variant.handedness();
        let x = match hand {
            Some(Handedness::Left) => {
                specs.scale + config.border
                    - specs.factors.treble_offset
                    - (specs.factors.x_ratio * self.length_treble)
            }
            _ => {
                specs.factors.treble_offset
                    + (specs.factors.x_ratio * self.length_treble)
                    + config.border
            }
        };
        let opposite = specs.factors.y_ratio * self.length_treble;
        let y = specs.bridge - opposite + config.border;
        Point(x, y)
    }
    /// Returns a Point struct containing both ends of a fret, nut or bridge
    /// which will form a line
    fn get_fret_line(&self, specs: &Specs, config: &Config) -> Line {
        let start = self.get_point_bass(specs, config);
        let end = self.get_point_treble(specs, config);
        Line { start, end }
    }
}

impl Line {
    /// Returns an svg Path node representing a single fret
    fn draw_fret(&self, fret: u32, config: &Config) -> svg::node::element::Path {
        let id = if fret == 0 {
            "Nut".to_string()
        } else {
            format!("Fret {}", fret)
        };
        let data = Data::new()
            .move_to((self.start.0, self.start.1))
            .line_to((self.end.0, self.end.1))
            .close();
        Path::new()
            .set("fill", "none")
            .set("stroke", config.fretline_color.to_hex())
            .set("stroke-opacity", config.fretline_color.alpha)
            .set("stroke-width", config.line_weight)
            .set("id", id)
            .set("d", data)
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
    factors: Factors,
}

impl Default for Specs {
    /// Returns a default Specs struct
    fn default() -> Self {
        Self::init(655.0, 24, Variant::default(), 43.0, 56.0, 8.0)
    }
}

impl Specs {
    #[must_use]
    pub fn init(
        scale: f64,
        count: u32,
        variant: Variant,
        nut: f64,
        bridge: f64,
        pfret: f64,
    ) -> Self {
        let factors = Factors::init(scale, &variant, nut, bridge, pfret);
        Self {
            scale,
            count,
            variant,
            nut,
            bridge,
            pfret,
            factors,
        }
    }

    pub fn builder() -> SpecsBuilder {
        SpecsBuilder::new()
    }

    /// Returns a multiscale Specs struct
    #[allow(clippy::must_use_candidate)]
    pub fn multi() -> Self {
        Self::init(655.0, 24, Variant::multi(), 43.0, 56.0, 8.0)
    }

    #[allow(clippy::must_use_candidate)]
    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    #[allow(clippy::must_use_candidate)]
    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn set_count(&mut self, count: u32) {
        self.count = count;
    }

    #[allow(clippy::must_use_candidate)]
    pub fn variant(&self) -> Variant {
        self.variant
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

    #[allow(clippy::must_use_candidate)]
    pub fn nut(&self) -> f64 {
        self.nut
    }

    pub fn set_nut(&mut self, nut: f64) {
        self.nut = nut;
    }

    #[allow(clippy::must_use_candidate)]
    pub fn bridge(&self) -> f64 {
        self.bridge
    }

    pub fn set_bridge(&mut self, bridge: f64) {
        self.bridge = bridge;
    }

    #[allow(clippy::must_use_candidate)]
    pub fn pfret(&self) -> f64 {
        self.pfret
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
    fn get_fret_lengths(&self, fret: u32) -> Lengths {
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
        let font_size = match config.units {
            Units::Metric => "5px",
            Units::Imperial => "0.25px",
        };
        line = format!("{} NutWidth: {:.2}{} |", line, self.nut, &units);
        let bridge = match config.units {
            Units::Metric => self.bridge - 6.0,
            Units::Imperial => self.bridge - (6.0 / 20.4),
        };
        line = format!("{} BridgeSpacing: {:.2}{}", line, bridge, &units);
        svg::node::element::Text::new()
            .set("x", config.border)
            .set("y", (config.border * 1.7) + self.bridge)
            .set("font-family", font_family)
            .set("font-weight", font_weight)
            .set("font-size", font_size)
            .set("id", "Specifications")
            .add(svg::node::Text::new(line))
    }

    /// Adds the centerline to the svg data
    fn draw_centerline(&self, config: &Config) -> svg::node::element::Path {
        let start_x = config.border;
        let start_y = (self.bridge / 2.0) + config.border;
        let end_x = config.border + self.scale;
        let end_y = (self.bridge / 2.0) + config.border;
        let (hex, opacity) = match &config.centerline_color {
            Some(c) => (c.to_hex(), f32::from(c.alpha) * 255.0),
            None => (RGBA::<u8>::primary(Blue).to_hex(), 1.0),
        };
        let dasharray = match config.units {
            Units::Metric => "4.0, 8.0",
            Units::Imperial => "0.2, 0.4",
        };
        let data = Data::new()
            .move_to((start_x, start_y))
            .line_to((end_x, end_y))
            .close();
        Path::new()
            .set("fill", "none")
            .set("stroke", hex)
            .set("stroke-opacity", opacity)
            .set("stroke-dasharray", dasharray)
            .set("stroke-dashoffset", "0")
            .set("stroke-width", config.line_weight)
            .set("id", "Centerline")
            .set("d", data)
    }

    /// adds the bridge as a line between the outer strings
    fn draw_bridge(&self, config: &Config) -> svg::node::element::Path {
        let start_x = match self.variant {
            Variant::Monoscale | Variant::Multiscale(_, Handedness::Right) => config.border,
            Variant::Multiscale(_, Handedness::Left) => config.border + self.scale,
        };
        let start_y = config.border;
        let end_x = match self.variant {
            Variant::Monoscale | Variant::Multiscale(_, Handedness::Right) => {
                config.border + self.factors.treble_offset
            }
            Variant::Multiscale(_, Handedness::Left) => {
                config.border + self.scale - self.factors.treble_offset
            }
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
    fn draw_fretboard(&self, config: &Config) -> svg::node::element::Path {
        let nut = self.get_nut().get_fret_line(self, config);
        let end = self
            .get_fret_lengths(self.count + 1)
            .get_fret_line(self, config);
        let (hex, alpha) = (
            config.fretboard_color.to_hex(),
            config.fretboard_color.alpha,
        );
        let data = Data::new()
            .move_to((nut.start.0, nut.start.1))
            .line_to((nut.end.0, nut.end.1))
            .line_to((end.end.0, end.end.1))
            .line_to((end.start.0, end.start.1))
            .line_to((nut.start.0, nut.start.1))
            .close();
        Path::new()
            .set("fill", hex)
            .set("fill-opacity", alpha)
            .set("stroke", "none")
            .set("id", "Fretboard")
            .set("d", data)
    }

    /// draws a single fret
    fn draw_fret(&self, config: &Config, num: u32) -> svg::node::element::Path {
        self.get_fret_lengths(num)
            .get_fret_line(self, config)
            .draw_fret(num, config)
    }

    /// Iterates through each fret, returning a group of svg Paths
    fn draw_frets(&self, cfg: &Config) -> svg::node::element::Group {
        let frets = Group::new().set("id", "Frets");
        let f: Vec<svg::node::element::Path> = (0..=self.count)
            .into_par_iter()
            .map(|fret| self.draw_fret(cfg, fret))
            .collect();
        f.into_iter().fold(frets, svg::node::element::Group::add)
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
    #[must_use]
    pub fn create_document(&self, conf: Option<Config>) -> svg::Document {
        let config = conf.unwrap_or_default();
        let width = (config.border * 2.0) + self.scale;
        let units = match config.units {
            Units::Metric => "mm",
            Units::Imperial => "in",
        };
        let widthmm = format!("{}{}", width, units);
        let height = (config.border * 2.0) + self.bridge;
        let heightmm = format!("{}{}", height, units);
        // Todo - investigate generating these values async
        let description = self.create_description();
        let fretboard = self.draw_fretboard(&config);
        let bridge = self.draw_bridge(&config);
        let frets = self.draw_frets(&config);
        let document = Document::new()
            .set("width", widthmm)
            .set("height", heightmm)
            .set("preserveAspectRatio", "xMidYMid meet")
            .set("viewBox", (0, 0, width, height))
            .add(description)
            .add(fretboard)
            .add(bridge)
            .add(frets);
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

/// A Specs builder
pub struct SpecsBuilder {
    scale: f64,
    count: u32,
    variant: Variant,
    nut: f64,
    bridge: f64,
    pfret: f64,
}

impl Default for SpecsBuilder {
    fn default() -> Self {
        Self {
            scale: 655.0,
            count: 24,
            variant: Variant::Monoscale,
            nut: 43.0,
            bridge: 56.0,
            pfret: 8.0,
        }
    }
}

impl SpecsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn nut(mut self, nut: f64) -> Self {
        self.nut = nut;
        self
    }

    pub fn bridge(mut self, bridge: f64) -> Self {
        self.bridge = bridge;
        self
    }

    pub fn pfret(mut self, pfret: f64) -> Self {
        self.pfret = pfret;
        self
    }

    pub fn build(self) -> Specs {
        Specs::init(self.scale, self.count, self.variant, self.nut, self.bridge, self.pfret)
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
        let val = var.scale();
        assert_eq!(val.unwrap(), 23.5);
        let hand = var.handedness();
        assert_eq!(hand.unwrap(), Handedness::Right);
    }

    #[test]
    fn factors_default() {
        let specs = Specs::default();
        assert_eq!(specs.factors.x_ratio, 0.9999507592328689);
        assert_eq!(specs.factors.y_ratio, 0.009923664122137405);
        assert_eq!(specs.factors.treble_offset, 0.0);
    }

    #[test]
    fn factors_multi() {
        let specs = Specs::multi();
        assert_eq!(specs.factors.x_ratio, 0.9999507592328689);
        assert_eq!(specs.factors.y_ratio, 0.009923664122137405);
        assert_eq!(specs.factors.treble_offset, 28.346827734356623);
    }

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
