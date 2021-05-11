#![warn(clippy::all, clippy::pedantic)]
use serde::{Deserialize, Serialize};

use crate::CONFIGDIR;

use std::path::{ Path, PathBuf };
use std::{ fs, process };

/// All of the configuration values which can be set in config.toml get stored
/// in this struct
#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    /// An external editor or viewer capable of handling svg image files
    pub external_program: String,
    /// The border which will appear around the rendering
    pub border: f64,
    /// The line weight for all of the elements in mm
    pub line_weight: f64,
    /// The color of the fret lines
    pub fretline_color: String,
    /// The background color of the fretboard
    pub fretboard_color: String,
    /// If true, draw a dashed horizontal centerline
    pub draw_centerline: bool,
    /// The color of the centerline
    pub centerline_color: String,
    /// Whether or not to print the fretboard specifications on the rendered svg
    pub print_specs: bool,
    /// The font used for the specifications
    pub font: Option<String>,
    /// The background color of the viewport for the preview image. This does not
    /// affect the final rendering, and changing it requires a restart to take
    /// effect
    pub background_color: String,
}

impl Config {
    /// Creates a [Config] struct with default values
    pub fn new() -> Config {
        Config {
            external_program: String::from("xdg-open"),
            border: 10.0,
            line_weight: 1.0,
            fretline_color: String::from("black"),
            fretboard_color: String::from("rgba(36,31,49,1)"),
            draw_centerline: true,
            centerline_color: String::from("blue"),
            print_specs: true,
            font: Some(String::from("Sans Regular 12")),
            background_color: String::from("rgba(255,255,255,1)"),
        }
    }

    /// Returns an OS appropriate configuration directory path
    pub fn get_config_dir() -> PathBuf {
        let mut configdir: PathBuf = match xdg_basedir::get_config_home() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        let progname = env!("CARGO_PKG_NAME");
        configdir.push(progname);
        if !configdir.exists() {
            fs::create_dir(&configdir.to_str().unwrap()).unwrap_or_else(|e| eprintln!("{}", e));
        }
        configdir
    }

    /// Returns the path to config.toml
    pub fn get_config_file() -> PathBuf {
        let mut file = CONFIGDIR.clone();
        file.push("config.toml");
        file
    }

    /// Deserializes config.toml into a [Config] struct
    pub fn from_file() -> Option<Config> {
        let config_file = Config::get_config_file();
        let config_file = if config_file.exists() {
            match fs::read_to_string(config_file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    return None;
                }
            }
        } else {
            return None;
        };
        let config: Config = match toml::from_str(&config_file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };
        Some(config)
    }

    /// Saves Template struct as a .toml file
    pub fn save_to_file(&self, file: &Path) {
        let toml_string = toml::to_string(&self).expect("Could not encode TOML value");
        fs::write(file, toml_string).expect("Could not write to file!");
    }
}
