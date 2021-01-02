#![warn(clippy::all, clippy::pedantic)]
use clap::{crate_version, load_yaml, App};
extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;
mod backend;
mod fretboard;
mod gui;
use backend::Specs;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).version(crate_version!()).get_matches();
    backend::run(&matches);
}
