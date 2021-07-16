use crate::{RGBA, Specs};
use crate::color::{HexColor, ReducedRGBA};

#[test]
fn black() {
    assert_eq!(RGBA::black().to_hex().color, HexColor::black().color);
    assert_eq!(ReducedRGBA::black().to_hex().color, HexColor::black().color);
}

#[test]
fn white() {
    assert_eq!(RGBA::white().to_hex().color, HexColor::white().color);
    assert_eq!(ReducedRGBA::white().to_hex().color, HexColor::white().color);
}

#[test]
fn red() {
    assert_eq!(RGBA::red().to_hex().color, HexColor::red().color);
    assert_eq!(ReducedRGBA::red().to_hex().color, HexColor::red().color);
}

#[test]
fn green() {
    assert_eq!(RGBA::green().to_hex().color, HexColor::green().color);
    assert_eq!(ReducedRGBA::green().to_hex().color, HexColor::green().color);
}

#[test]
fn blue() {
    assert_eq!(RGBA::blue().to_hex().color, HexColor::blue().color);
    assert_eq!(ReducedRGBA::blue().to_hex().color, HexColor::blue().color);
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
