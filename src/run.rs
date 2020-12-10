#![warn(clippy::all, clippy::pedantic)]
use crate::fretboard::Lengths;
use rug::ops::Pow;
use std::process::Command;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;

pub struct Specs {
    pub scale: f64,
    pub count: u32,
    pub multi: bool,
    pub scale_treble: f64,
    pub nut: f64,
    pub bridge: f64,
    pub pfret: usize,
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
    fn get_factors(&self, fretboard: &[Lengths]) -> Factors {
        let height = (self.bridge - self.nut) / 2.0;
        let y_ratio = height / self.scale;
        let x_ratio = y_ratio.acos().sin();
        let bass_pfret = x_ratio * fretboard[self.pfret].length_bass;
        let treble_pfret = x_ratio * fretboard[self.pfret].length_treble;
        let treble_offset = bass_pfret - treble_pfret;
        Factors {
            x_ratio,
            y_ratio,
            treble_offset,
        }
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
            .set("stroke-dasharray", "4.0,8.0")
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
    fn create_document(&self, factors: &Factors, fretboard: &[Lengths]) {
        let width = (self.border * 2.0) + self.scale;
        let widthmm = format!("{}mm", width);
        let height = (self.border * 2.0) + self.bridge;
        let heightmm = format!("{}mm", height);
        let mut document = Document::new()
            .set("width", widthmm)
            .set("height", heightmm)
            .set("preserveAspectRatio", "xMidYMid meet")
            .set("viewBox", (0, 0, width, height));
        document = document.add(self.draw_centerline());
        document = document.add(self.draw_fretboard(&fretboard, &factors));
        document = document.add(self.draw_bridge(&factors));
        document = document.add(self.draw_frets(&fretboard, &factors));
        svg::save(&self.output, &document).unwrap();
    }
    pub fn run(&self) {
        let lengths: Vec<Lengths> = self.get_all_lengths();
        let factors = &self.get_factors(&lengths);
        self.create_document(factors, &lengths);
        println!("Output saved as {}.", self.output);
        if self.external {
            Command::new(&self.cmd)
                .args(&[&self.output])
                .spawn()
                .unwrap();
        }
    }
}
