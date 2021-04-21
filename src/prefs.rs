#![warn(clippy::all, clippy::pedantic)]
use gio::AppInfoExt;
use glib::clone;
use gtk;
use gtk::prelude::*;
use serde::{ Deserialize, Serialize };
use xdg_basedir::*;

use crate::CONFIGDIR;

use std::{env, fs, process};
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    external_program: Option<PathBuf>,
    border: f64,
    line_weight: f64,
    fretline_color: String,
    draw_centerline: bool,
    centerline_color: String,
    print_specs: bool,
    font: Option<String>,
    background_color: String,
}

struct PrefWidgets {
    prefs_window: gtk::Dialog,
    external_program: gtk::AppChooserButton,
    border: gtk::SpinButton,
    line_weight: gtk::SpinButton,
    fretline_color: gtk::ColorButton,
    draw_centerline: gtk::Switch,
    centerline_color: gtk::ColorButton,
    print_specs: gtk::Switch,
    font_chooser: gtk::FontButton,
    background_color: gtk::ColorButton,
}

impl PrefWidgets {
    fn new() -> PrefWidgets {
        let glade_src = include_str!("prefs.glade");
        let builder = gtk::Builder::from_string(glade_src);
        PrefWidgets {
            prefs_window: builder.get_object("prefs_window").expect("Error getting 'prefs_window'"),
            external_program: builder.get_object("external_program").expect("Error getting 'external_program'"),
            border: builder.get_object("border").expect("Error getting 'border'"),
            line_weight: builder.get_object("line_weight").expect("Error getting 'line_weight'"),
            fretline_color: builder.get_object("fretline_color").expect("Error getting 'fretline_color'"),
            draw_centerline: builder.get_object("draw_centerline").expect("Error getting 'draw_centerline'"),
            centerline_color: builder.get_object("centerline_color").expect("Error getting 'centerline_color'"),
            print_specs: builder.get_object("print_specs").expect("Error getting 'print_specs'"),
            font_chooser: builder.get_object("font_chooser").expect("Error getting 'font_chooser'"),
            background_color: builder.get_object("background_color").expect("Error getting 'background_color'"),
        }
    }

    fn get_color_string(&self, button: &gtk::ColorButton) -> String {
        let color = button.get_rgba();
        format!("rgba({},{},{},{})",
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
            (color.alpha * 255.0) as u8
        )
    }

    fn config_from_widgets(&self) -> Config {
        Config {
            external_program: match self.external_program.get_app_info() {
                Some(c) => c.get_commandline(),
                None => None,
            },
            border: self.border.get_value(),
            line_weight: self.line_weight.get_value(),
            fretline_color: self.get_color_string(&self.fretline_color),
            draw_centerline: self.draw_centerline.get_active(),
            centerline_color: self.get_color_string(&self.centerline_color),
            print_specs: self.print_specs.get_active(),
            font: {
                match self.font_chooser.get_font() {
                    Some(c) => Some(String::from(c)),
                    None => None,
                }
            },
            background_color: self.get_color_string(&self.background_color),
        }
    }

    fn load_config(&self) {
        if let Some(config) = Config::from_file() {
            if let Ok(color) = gdk::RGBA::from_str(&config.fretline_color) {
                self.fretline_color.set_rgba(&color);
            }
            if let Ok(color) = gdk::RGBA::from_str(&config.centerline_color) {
                self.centerline_color.set_rgba(&color);
            }
            if let Ok(color) = gdk::RGBA::from_str(&config.background_color) {
                self.background_color.set_rgba(&color);
            }
            self.border.set_value(config.border);
            self.line_weight.set_value(config.line_weight);
            self.draw_centerline.set_active(config.draw_centerline);
            self.print_specs.set_active(config.print_specs);
            if let Some(font) = config.font {
                self.font_chooser.set_font(&font);
            }
        }
    }

    fn save_prefs(&self) {
        let config_file = Config::get_config_file();
        let config_data = self.config_from_widgets();
        config_data.save_to_file(&config_file);
    }
}

impl Config {
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
        if !configdir.exists() {
            fs::create_dir(&configdir.to_str().unwrap()).unwrap_or_else(|e|
                eprintln!("{}", e));
        }
        configdir
    }

    fn get_config_file() -> PathBuf {
        let mut file = CONFIGDIR.clone();
        file.push("config.toml");
        file
    }

    fn from_file() -> Option<Config> {
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
                return None
            }
        };
        Some(config)
    }

    /// Saves Template struct as a .toml file
    pub fn save_to_file(&self, file: &PathBuf) {
        let toml_string = toml::to_string(&self).expect("Could not encode TOML value");
        fs::write(file, toml_string).expect("Could not write to file!");
    }
}

pub fn run() {
    let prefs = Rc::new(PrefWidgets::new());
    prefs.load_config();
    prefs.external_program
        .connect_changed(clone!(@weak prefs => move |_| {
            if let Some(_) = prefs.external_program.get_app_info() {
                prefs.save_prefs();
            }
        }));

    prefs.border
        .connect_value_changed(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    prefs.line_weight
        .connect_value_changed(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    prefs.fretline_color
        .connect_color_set(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    let prefs_clone = prefs.clone();
    prefs.draw_centerline
        .connect_state_set( move |_,_| {
            prefs_clone.save_prefs();
            gtk::Inhibit(false)
        });

    prefs.centerline_color
        .connect_color_set(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    let prefs_clone = prefs.clone();
    prefs.print_specs
        .connect_state_set( move |_,_| {
            prefs_clone.save_prefs();
            gtk::Inhibit(false)
        });

    prefs.font_chooser
        .connect_font_set(clone!(@weak prefs => move |_| {
            if let Some(_) = prefs.font_chooser.get_font() {
                prefs.save_prefs();
            }
        }));

    prefs.background_color
        .connect_color_set(clone!(@weak prefs => move |_| {
            prefs.save_prefs();
        }));

    prefs.prefs_window.run();
    prefs.prefs_window.close();
}
