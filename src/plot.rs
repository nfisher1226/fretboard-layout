#![warn(clippy::all, clippy::pedantic)]
use crate::fretboard::Fret;
use crate::Specs;
use std::f64;

pub struct Factors {
    pub x_ratio: f64,
    pub y_ratio: f64,
    pub treble_offset: f64,
}

pub struct Point (pub f64, pub f64);

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Factors {
    pub fn get_factors(fretboard: &[Fret], specs: &Specs) -> Factors {
        let height = (specs.bridge - specs.nut) / 2.0;
        let y_ratio = height / specs.scale;
        let x_ratio = y_ratio.acos().sin();
        let bass_pfret = x_ratio * fretboard[specs.pfret].ftob_bass;
        let treble_pfret = x_ratio * fretboard[specs.pfret].ftob_treble;
        let treble_offset = bass_pfret - treble_pfret;
        Factors {
            x_ratio,
            y_ratio,
            treble_offset,
        }
    }
}

impl Point {
    fn get_point_bass(fretboard: &[Fret], fret: usize, factors: &Factors, specs: &Specs) -> Point {
        let x = (factors.x_ratio * fretboard[fret].ftob_bass) + specs.border;
        let y = (factors.y_ratio * fretboard[fret].ftob_bass) + specs.border;
        Point (x, y)
    }
    fn get_point_treble(fretboard: &[Fret], fret: usize, factors: &Factors, specs: &Specs) -> Point {
        let x = factors.treble_offset + (factors.x_ratio * fretboard[fret].ftob_treble) + specs.border;
        let y = specs.bridge - (factors.y_ratio * fretboard[fret].ftob_treble) + specs.border;
        Point (x, y)
    }
}

impl Line {
    pub fn get_fret_line(fretboard: &[Fret], fret: usize, factors: &Factors, specs: &Specs) -> Line {
        let start = Point::get_point_bass(&fretboard, fret, &factors, &specs);
        let end = Point::get_point_treble(&fretboard, fret, &factors, &specs);
        Line {
            start,
            end,
        }
    }
}
