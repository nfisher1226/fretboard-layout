#![warn(clippy::all, clippy::pedantic)]
use serde::Deserialize;
use xdg_basedir::*;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::{env, process};

use crate::CONFIGDIR;

#[derive(Deserialize, Debug)]
pub struct Config {
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

