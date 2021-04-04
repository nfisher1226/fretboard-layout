#![warn(clippy::all, clippy::pedantic)]
//! Gfret renders an svg image template of a fretboard for a stringed instrument.
//! It has a Gtk interface as well as a command line interface and can produce
//! templates for instruments ranging from a piccolo mandolin to an upright bass.
//! Multiscale designs are also supported.
use clap::{crate_version, load_yaml, App};
/// Processes the data provided by the gui into a fully rendered svg image.
mod backend;
/// Used by the backend to calculate point locations and lines.
mod fretboard;
/// The Gtk user interface to gfret.
mod gui;
/// Persistent templates
mod template;

use backend::Specs;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).version(crate_version!()).get_matches();
    backend::run(&matches);
}
