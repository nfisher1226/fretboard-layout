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
use std::process;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).version(crate_version!()).get_matches();

    match matches.subcommand() {
        Some(("cli", cli_matches)) => {
            let scale: f64 = match cli_matches.value_of_t("SCALE") {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                },
            };
            let specs = Specs {
                scale,
                count: match cli_matches.value_of_t("COUNT") {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    },
                },
                multi: cli_matches.occurrences_of("MULTI") > 0,
                scale_treble: if cli_matches.occurrences_of("MULTI") > 0 {
                    match cli_matches.value_of_t("MULTI") {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("{}", e);
                            process::exit(1);
                        },
                    }
                } else {
                    scale
                },
                nut: match cli_matches.value_of_t("NUT") {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    },
                },
                bridge: match cli_matches.value_of_t::<f64>("BRIDGE") {
                    Ok(c) => c + 6.0,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    },
                },
                pfret: match cli_matches.value_of_t("PERPENDICULAR") {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    },
                },
                output: cli_matches.value_of("OUTPUT").unwrap().to_string(),
                border: match cli_matches.value_of_t("BORDER") {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    },
                },
                external: cli_matches.occurrences_of("EXTERN") > 0,
                cmd: cli_matches.value_of("EXTERN").unwrap().to_string(),
            };
            specs.run();
        }
        _ => gui::run_gui(),
    }
}
