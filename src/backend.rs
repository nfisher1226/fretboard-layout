#![warn(clippy::all, clippy::pedantic)]
use crate::fretboard::Lengths;
use clap::ArgMatches;
use rug::ops::Pow;
use std::process;
use std::process::Command;
use svg::node::element::{path::Data, Description, Group, Path};
use svg::Document;

pub struct Specs {
    pub scale: f64,
    pub count: u32,
    pub multi: bool,
    pub scale_treble: f64,
    pub nut: f64,
    pub bridge: f64,
    pub pfret: f64,
    pub output: String,
    pub border: f64,
    pub external: bool,
    pub cmd: String,
}

pub struct Factors {
    pub x_ratio: f64,
    pub y_ratio: f64,
    pub treble_offset: f64,
}

impl Specs {
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
    fn create_description(&self) -> svg::node::element::Description {
        Description::new()
            .set("Scale", self.scale)
            .set("Multiscale", self.multi)
            .set("ScaleTreble", self.scale_treble)
            .set("PerpendicularFret", self.pfret)
            .set("BridgeSpacing", self.bridge - 6.0)
            .set("NutWidth", self.nut)
    }
    fn print_data(&self) -> svg::node::element::Text {
        let mut line = if self.multi {
            format!(
                "ScaleBass: {:.2}mm | ScaleTreble: {:.2}mm |",
                self.scale, self.scale_treble
            )
        } else {
            format!("Scale: {:.2}mm |", self.scale)
        };
        line = format!("{} PerpendicularFret: {:.1} |", line, self.pfret);
        line = format!("{} NutWidth: {:.2}mm |", line, self.nut);
        line = format!("{} BridgeSpacing: {:.2}mm", line, self.bridge - 6.0);
        svg::node::element::Text::new()
            .set("x", self.border)
            .set("y", (self.border * 1.7) + self.bridge)
            .set("font-family", "sans-serif")
            .set("font-weight", "normal")
            .set("font-size", "5px")
            .set("id", "Specifications")
            .add(svg::node::Text::new(line))
    }
    fn draw_centerline(&self) -> svg::node::element::Path {
        let start_x = self.border;
        let start_y = (self.bridge / 2.0) + self.border;
        let end_x = self.border + self.scale;
        let end_y = (self.bridge / 2.0) + self.border;
        let data = Data::new()
            .move_to((start_x, start_y))
            .line_to((end_x, end_y))
            .close();
        Path::new()
            .set("fill", "none")
            .set("stroke", "blue")
            .set("stroke-dasharray", "4.0, 8.0")
            .set("stroke-dashoffset", "0")
            .set("stroke-width", 1)
            .set("id", "Centerline")
            .set("d", data)
    }
    fn draw_bridge(&self, factors: &Factors) -> svg::node::element::Path {
        let start_x = self.border;
        let start_y = self.border;
        let end_x = self.border + factors.treble_offset;
        let end_y = self.border + self.bridge;
        let data = Data::new()
            .move_to((start_x, start_y))
            .line_to((end_x, end_y))
            .close();
        Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("id", "Bridge")
            .set("d", data)
    }
    fn draw_fretboard(&self, fretboard: &[Lengths], factors: &Factors) -> svg::node::element::Path {
        let nut = fretboard[0_usize].get_fret_line(&factors, &self);
        let end = fretboard[self.count as usize + 1].get_fret_line(&factors, &self);
        let data = Data::new()
            .move_to((nut.start.0, nut.start.1))
            .line_to((nut.end.0, nut.end.1))
            .line_to((end.end.0, end.end.1))
            .line_to((end.start.0, end.start.1))
            .line_to((nut.start.0, nut.start.1))
            .close();
        Path::new()
            .set("fill", "none")
            .set("stroke", "grey")
            .set("stroke-width", 1)
            .set("id", "Fretboard")
            .set("d", data)
    }
    fn draw_frets(&self, fretboard: &[Lengths], factors: &Factors) -> svg::node::element::Group {
        let mut frets = Group::new().set("id", "Frets");
        for fret in 0..=self.count {
            let line = fretboard[fret as usize].get_fret_line(&factors, &self);
            frets = frets.add(line.draw_fret(fret));
        }
        frets
    }
    pub fn create_document(&self) -> svg::Document {
        let lengths: Vec<Lengths> = self.get_all_lengths();
        let factors = &self.get_factors();
        let width = (self.border * 2.0) + self.scale;
        let widthmm = format!("{}mm", width);
        let height = (self.border * 2.0) + self.bridge;
        let heightmm = format!("{}mm", height);
        Document::new()
            .set("width", widthmm)
            .set("height", heightmm)
            .set("preserveAspectRatio", "xMidYMid meet")
            .set("viewBox", (0, 0, width, height))
            .add(self.create_description())
            .add(self.print_data())
            .add(self.draw_centerline())
            .add(self.draw_fretboard(&lengths, &factors))
            .add(self.draw_bridge(&factors))
            .add(self.draw_frets(&lengths, &factors))
    }
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
    match matches.subcommand() {
        Some(("cli", cli_matches)) => {
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
                border: match cli_matches.value_of_t("BORDER") {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                },
                external: cli_matches.occurrences_of("EXTERN") > 0,
                cmd: cli_matches.value_of("EXTERN").unwrap().to_string(),
            };
            specs.run();
        }
        _ => crate::gui::run_ui(),
    }
}
