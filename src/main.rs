#![warn(clippy::all, clippy::pedantic)]
use clap::{App, load_yaml};
extern crate gtk;
extern crate gio;
extern crate gdk_pixbuf;
mod fretboard;
mod gui;
mod backend;
use backend::Specs;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    match matches.subcommand() {
        Some(("cli", cli_matches)) => {
            let multi = cli_matches.occurrences_of("MULTI") > 0;
            let scale_treble: f64 = if multi {
                cli_matches.value_of_t("MULTI").unwrap()
            } else {
                cli_matches.value_of_t("SCALE").unwrap()
            };
            let cmd = cli_matches.value_of("EXTERN").unwrap().to_string();
            let bridge: f64 = cli_matches.value_of_t("BRIDGE").unwrap();
            let specs = Specs {
                scale: cli_matches.value_of_t("SCALE").unwrap(),
                count: cli_matches.value_of_t("COUNT").unwrap(),
                multi,
                scale_treble,
                nut: cli_matches.value_of_t("NUT").unwrap(),
                bridge: bridge + 6.0,
                pfret: cli_matches.value_of_t("PERPENDICULAR").unwrap(),
                output: cli_matches.value_of("OUTPUT").unwrap().to_string(),
                border: cli_matches.value_of_t("BORDER").unwrap(),
                external: cli_matches.occurrences_of("EXTERN") > 0,
                cmd,
            };
            specs.run();
        }
        _ => gui::run_gui(),
    }
}
