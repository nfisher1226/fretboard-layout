#![warn(clippy::all, clippy::pedantic)]
use crate::gui::Gui;
use serde::Deserialize;
use xdg_basedir::*;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::{env, process};

#[derive(Deserialize, Debug)]
pub struct Template {
    pub scale: f64,
    pub count: u32,
    pub scale_treble: Option<f64>,
    pub nut: f64,
    pub bridge: f64,
    pub pfret: Option<f64>,
    pub border: Option<f64>,
}

pub fn get_config_dir() -> PathBuf {
    let mut configdir: PathBuf = match get_config_home() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let progname = env!("CARGO_PKG_NAME");
    configdir.push(progname);
    configdir
}

impl Template {
    pub fn load_from_toml(file: PathBuf) -> Option<Template> {
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
}
