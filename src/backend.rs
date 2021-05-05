#![warn(clippy::all, clippy::pedantic)]
use crate::fretboard::Lengths;
use crate::prefs::Config;
use clap::ArgMatches;
use pango::FontDescription;
use rug::ops::Pow;
use std::process;
use std::process::Command;
use svg::node::element::{path::Data, Description, Group, Path};
use svg::Document;

/// This struct contains the user data used to create the svg output file
pub struct Specs {
    /// Scale length. For multiscale designs this is the bass side scale length.
    pub scale: f64,
    /// Number of frets to render
    pub count: u32,
    /// True if the design is multiscale
    pub multi: bool,
    /// The scale length for the treble side. Ignored for single scale designs.
    pub scale_treble: f64,
    /// The width of the fretboard at the nut.
    pub nut: f64,
    /// The string spacing at the bridge. Note that this is not the physical
    /// width of the bridge, but the distance perpendicular to the centerline
    /// between the outer two strings.
    pub bridge: f64,
    /// The fret that is perpendicular to the centerline.
    pub pfret: f64,
    /// An output file, '-' for stdout.
    pub output: String,
    /// Whether to open the rendered image in an external program.
    pub external: bool,
    /// The external program to open.
    pub cmd: String,
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
    /// Returns the distance from bridge to nut on both sides of the fretboard
    fn get_nut(&self) -> Lengths {
        let length_treble = if self.multi {
            self.scale_treble
        } else {
            self.scale
        };
        Lengths {
            length_bass: self.scale,
            length_treble,
        }
    }

    /// Returns the length from bridge to fret for a given fret number, along
    /// both bass and treble sides of the board.
    fn get_fret_lengths(&self, fret: u32) -> Lengths {
        let factor = 2.0_f64.pow(f64::from(fret) / 12.0);
        let length_bass = self.scale / factor;
        let length_treble = if self.multi {
            self.scale_treble / factor
        } else {
            length_bass
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
        let factor = 2.0_f64.pow(self.pfret / 12.0);
        let length_bass = self.scale / factor;
        let length_treble = if self.multi {
            self.scale_treble / factor
        } else {
            length_bass
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
            .set("Multiscale", self.multi)
            .set("ScaleTreble", self.scale_treble)
            .set("PerpendicularFret", self.pfret)
            .set("BridgeSpacing", self.bridge - 6.0)
            .set("NutWidth", self.nut)
    }

    /// Prints the specs used in the rendered image
    fn print_data(&self, config: &Config) -> svg::node::element::Text {
        let mut line = if self.multi {
            format!(
                "ScaleBass: {:.2}mm | ScaleTreble: {:.2}mm | PerpendicularFret: {:.1} |",
                self.scale, self.scale_treble, self.pfret
            )
        } else {
            format!("Scale: {:.2}mm |", self.scale)
        };
        let font_string = match &config.font {
            Some(font) => font.to_string(),
            None => String::from("Sans Regular 12"),
        };
        let font = FontDescription::from_string(&font_string);
        let font_family = match font.get_family() {
            Some(fam) => fam.to_string(),
            None => String::from("sans-serif"),
        };
        let font_weight = font.get_style().to_string();
        line = format!("{} NutWidth: {:.2}mm |", line, self.nut);
        line = format!("{} BridgeSpacing: {:.2}mm", line, self.bridge - 6.0);
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
        let data = Data::new()
            .move_to((start_x, start_y))
            .line_to((end_x, end_y))
            .close();
        Path::new()
            .set("fill", "none")
            .set("stroke", config.centerline_color.as_str())
            .set("stroke-dasharray", "4.0, 8.0")
            .set("stroke-dashoffset", "0")
            .set("stroke-width", config.line_weight)
            .set("id", "Centerline")
            .set("d", data)
    }

    /// adds the bridge as a line between the outer strings
    fn draw_bridge(&self, factors: &Factors, config: &Config) -> svg::node::element::Path {
        let start_x = config.border;
        let start_y = config.border;
        let end_x = config.border + factors.treble_offset;
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
        let data = Data::new()
            .move_to((nut.start.0, nut.start.1))
            .line_to((nut.end.0, nut.end.1))
            .line_to((end.end.0, end.end.1))
            .line_to((end.start.0, end.start.1))
            .line_to((nut.start.0, nut.start.1))
            .close();
        Path::new()
            .set("fill", config.fretboard_color.clone())
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

    /// Returns the complete svg Document
    pub fn create_document(&self) -> svg::Document {
        let config = match Config::from_file() {
            Some(c) => c,
            None => Config::new(),
        };
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
        if config.print_specs {
            if config.draw_centerline {
                document
                    .add(self.print_data(&config))
                    .add(self.draw_centerline(&config))
            } else {
                document.add(self.print_data(&config))
            }
        } else if config.draw_centerline {
            document.add(self.draw_centerline(&config))
        } else {
            document
        }
    }

    /// Gets the document and saves output
    pub fn run(&self) {
        let document = self.create_document();
        if self.output == "-" {
            println!("{}", document);
        } else {
            match svg::save(&self.output, &document) {
                Ok(_) => println!("Output saved as {}.", self.output),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            };
            if self.external {
                match Command::new(&self.cmd).args(&[&self.output]).spawn() {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}

pub fn run(matches: &ArgMatches) {
    if let Some(("cli", cli_matches)) = matches.subcommand() {
        let scale: f64 = match cli_matches.value_of_t("SCALE") {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        let specs = Specs {
            scale,
            count: match cli_matches.value_of_t("COUNT") {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            },
            multi: cli_matches.occurrences_of("MULTI") > 0,
            scale_treble: if cli_matches.occurrences_of("MULTI") > 0 {
                match cli_matches.value_of_t("MULTI") {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
            } else {
                scale
            },
            nut: match cli_matches.value_of_t("NUT") {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            },
            bridge: match cli_matches.value_of_t::<f64>("BRIDGE") {
                Ok(c) => c + 6.0,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            },
            pfret: match cli_matches.value_of_t("PERPENDICULAR") {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            },
            output: cli_matches.value_of("OUTPUT").unwrap().to_string(),
            external: cli_matches.occurrences_of("EXTERN") > 0,
            cmd: cli_matches.value_of("EXTERN").unwrap().to_string(),
        };
        specs.run();
    } else {
        let template = if matches.occurrences_of("TEMPLATE") > 0 {
            matches.value_of("TEMPLATE")
        } else {
            None
        };
        crate::gui::run_ui(template);
    }
}
