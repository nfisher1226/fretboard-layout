#![warn(clippy::all, clippy::pedantic)]
use crate::CONFIGDIR;
use crate::gui::Gui;
use serde::{ Deserialize, Serialize };
use toml;
use xdg_basedir::*;

use std::fs;
use std::path::PathBuf;
use std::{env, process};

#[derive(Deserialize, Debug, Serialize)]
pub struct Template {
    pub scale: f64,
    pub count: u32,
    pub scale_treble: Option<f64>,
    pub nut: f64,
    pub bridge: f64,
    pub pfret: Option<f64>,
    pub border: Option<f64>,
}

impl Template {
    /// Takes a filename as an argument and returns either a populated Template
    /// struct, or else None.
    pub fn load_from_file(file: PathBuf) -> Option<Template> {
        let file = if file.exists() {
            match fs::read_to_string(file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    return None;
                }
            }
        } else {
            return None;
        };
        let template: Template = match toml::from_str(&file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                return None
            }
        };
        Some(template)
    }
    /// Saves Template struct as a .toml file
    fn save_to_file(&self, file: &PathBuf) {
        let toml_string = toml::to_string(&self).expect("Could not encode TOML value");
        let mut file = file.clone();
        file.set_extension("toml");
        fs::write(file, toml_string).expect("Could not write to file!");
    }
    /// Saves the program state on exit
    pub fn save_statefile(&self) {
        let mut statefile = CONFIGDIR.clone();
        statefile.push("state.toml");
        self.save_to_file(&statefile);
    }
}
