#![warn(clippy::all, clippy::pedantic)]
use rug::ops::Pow;
use crate::Specs;

pub struct Fret {
    pub ftob_bass: f64,
    pub ftob_treble: f64,
}

impl Fret {
    fn get_fret(fret: i32, specs: &Specs) -> Fret {
        let factor = 2.0_f64.pow(f64::from(fret) / 12.0);
        let ftob_bass = specs.scale / factor;
        let ftob_treble = if specs.multi {
            specs.scale_treble / factor
        } else {
            ftob_bass
        };
        Fret {
            ftob_bass,
            ftob_treble,
        }
    }
    fn get_nut(specs: &Specs) -> Fret {
        let ftob_treble = if specs.multi {
            specs.scale_treble
        } else {
            specs.scale
        };
        Fret {
            ftob_bass: specs.scale,
            ftob_treble,
        }
    }
    pub fn get_fretboard(specs: &Specs) -> Vec<Fret> {
        let mut fretboard: Vec<Fret> = Vec::new();
        let nut = Fret::get_nut(&specs);
        fretboard.push(nut);
        for n in 1..specs.count + 2 {
            let fret = Fret::get_fret(n, specs);
            fretboard.push(fret);
        }
        fretboard
    }
}
