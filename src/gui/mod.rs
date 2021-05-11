#![warn(clippy::all, clippy::pedantic)]
use crate::template::Template;
use crate::Specs;
use crate::CONFIGDIR;
/// Preferences Dialog and configuration data
pub mod prefs;
use crate::Config;
use clap::crate_version;
use gdk_pixbuf::Pixbuf;
use gio::{Cancellable, MemoryInputStream};
use glib::clone;
use gtk::{
    prelude::*, DialogExt, FileChooserAction, FileChooserExt, Inhibit, RangeExt, ResponseType,
    ResponseType::Accept, SpinButtonExt, ToggleButtonExt, WidgetExt, Window, WindowType,
};

use std::cell::RefCell;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;

/// The Gui struct keeps track of our widgets as a group to provide access to
/// the data which they represent for multiple functions.
pub struct Gui {
    image_preview: gtk::Image,
    pub scale: gtk::Scale,
    pub checkbox_multi: gtk::CheckButton,
    pub scale_multi_course: gtk::Scale,
    scale_multi_fine: gtk::SpinButton,
    pub fret_count: gtk::SpinButton,
    pfret_label: gtk::Label,
    pub perpendicular_fret: gtk::SpinButton,
    pub nut_width: gtk::SpinButton,
    pub bridge_spacing: gtk::SpinButton,
    saved_once: RefCell<bool>,
    saved_current: RefCell<bool>,
    filename: RefCell<String>,
    open_template: gtk::MenuItem,
    save_file: gtk::MenuItem,
    save_as: gtk::MenuItem,
    open_external: gtk::MenuItem,
    preferences: gtk::MenuItem,
    quit: gtk::MenuItem,
    window: gtk::Window,
}

impl Gui {
    /// Returns a struct containing pointers to the widgets which keep program
    /// state, as well as further state contained within [RefCell] containers
    fn new() -> Rc<Gui> {
        let glade_src = include_str!("ui.glade");
        let builder = gtk::Builder::from_string(glade_src);

        Rc::new(Gui {
            image_preview: builder.get_object("image_preview").unwrap(),
            scale: builder.get_object("scale_course").unwrap(),
            checkbox_multi: builder.get_object("check_box_multi").unwrap(),
            scale_multi_course: builder.get_object("scale_multi_course").unwrap(),
            scale_multi_fine: builder.get_object("scale_multi_fine").unwrap(),
            fret_count: builder.get_object("fret_count").unwrap(),
            perpendicular_fret: builder.get_object("perpendicular_fret").unwrap(),
            pfret_label: builder.get_object("pfret_label").unwrap(),
            nut_width: builder.get_object("nut_width").unwrap(),
            bridge_spacing: builder.get_object("bridge_spacing").unwrap(),
            saved_once: RefCell::new(false),
            saved_current: RefCell::new(false),
            filename: RefCell::new(String::from("")),
            open_template: builder.get_object("open_template").unwrap(),
            save_file: builder.get_object("save_file").unwrap(),
            save_as: builder.get_object("save_as").unwrap(),
            open_external: builder.get_object("open_external").unwrap(),
            preferences: builder.get_object("preferences").unwrap(),
            quit: builder.get_object("quit").unwrap(),
            window: builder.get_object("mainWindow").unwrap(),
        })
    }

    /// Sets widget state to match temmplate
    pub fn load_template(&self, template: &Template) {
        self.scale.set_value(template.scale);
        self.fret_count.set_value(template.count.into());
        if let Some(scale_treble) = template.scale_treble {
            self.scale_multi_course.set_value(scale_treble);
            self.checkbox_multi.set_active(true);
        } else {
            self.checkbox_multi.set_active(false);
        }
        self.toggle_multi();
        self.nut_width.set_value(template.nut);
        self.bridge_spacing.set_value(template.bridge);
        if let Some(pfret) = template.pfret {
            self.perpendicular_fret.set_value(pfret);
        }
    }

    /// Populates an instance of Template from the gui
    #[allow(clippy::cast_sign_loss)]
    fn template_from_gui(&self) -> Template {
        Template {
            scale: self.scale.get_value(),
            count: self.fret_count.get_value_as_int() as u32,
            scale_treble: {
                if self.checkbox_multi.get_active() {
                    Some(self.scale_multi_course.get_value())
                } else {
                    None
                }
            },
            nut: self.nut_width.get_value(),
            bridge: self.bridge_spacing.get_value(),
            pfret: Some(self.perpendicular_fret.get_value()),
        }
    }

    /// Takes the data represented by our Gtk widgets and outputs a Specs struct
    /// which will be used by the backend to render the svg image.
    #[allow(clippy::cast_sign_loss)]
    fn get_specs(&self, filename: &str) -> Specs {
        Specs {
            scale: self.scale.get_value(),
            count: self.fret_count.get_value_as_int() as u32,
            multi: self.checkbox_multi.get_active(),
            scale_treble: self.scale_multi_course.get_value(),
            nut: self.nut_width.get_value(),
            bridge: self.bridge_spacing.get_value() + 6.0,
            pfret: self.perpendicular_fret.get_value(),
            output: filename.to_string(),
            external: false,
            cmd: String::from(""),
        }
    }

    /// Performs a full render of the svg image without saving to disk, and
    /// refreshes the image preview with the new data.
    fn draw_preview(&self, swap: bool) {
        let image = self.get_specs("-").create_document().to_string();
        let bytes = glib::Bytes::from_owned(image.into_bytes());
        let stream = gio::MemoryInputStream::from_bytes(&bytes);
        let window_size = self.window.get_size();
        let pixbuf = Pixbuf::from_stream_at_scale::<MemoryInputStream, Cancellable>(
            &stream,
            window_size.0,
            -1,
            true,
            None,
        );
        self.image_preview.set_from_pixbuf(Some(&pixbuf.unwrap()));
        if swap {
            self.saved_current.swap(&RefCell::new(false));
            self.set_window_title();
        }
    }

    /// Toggles certain gui elements on and off when we switch from
    /// single scale to multiscale and back again.
    fn toggle_multi(&self) {
        let value = self.checkbox_multi.get_active();
        self.scale_multi_course.set_sensitive(value);
        self.scale_multi_fine.set_sensitive(value);
        if value {
            self.perpendicular_fret.show();
            self.pfret_label.show();
        } else {
            self.perpendicular_fret.hide();
            self.pfret_label.hide();
        }
    }

    /// Saves the file and opens it with an external program.
    fn open_external(&self) {
        if !*self.saved_current.borrow() {
            self.save_file();
        }
        if *self.saved_current.borrow() {
            let cmd = if let Some(config) = Config::from_file() {
                config.external_program
            } else {
                String::from("xdg-open")
            };
            let filename = self.filename.borrow().to_string();
            Command::new(cmd).args(&[&filename]).spawn().unwrap();
        }
    }

    /// Opens a [gtk::FileChooserDialog] and sets the path to the output file.
    fn get_output(&self) -> Option<String> {
        let currentfile = if *self.saved_once.borrow() {
            self.filename.borrow().to_string()
        } else {
            String::from("untitled.svg")
        };
        let dialog = gtk::FileChooserDialog::with_buttons::<Window>(
            Some("Save As"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
            &[
                ("_Cancel", ResponseType::Cancel),
                ("_Ok", ResponseType::Accept),
            ],
        );
        dialog.set_current_name(&currentfile);
        dialog.set_do_overwrite_confirmation(true);
        let res = dialog.run();
        let filename: Option<String> = if res == Accept {
            dialog.get_filename().and_then(|mut name| {
                name.set_extension("svg");
                match name.to_str() {
                    Some(c) => Some(c.to_string()),
                    None => Some(currentfile),
                }
            })
        } else {
            None
        };
        dialog.close();
        filename
    }

    /// Determines if the file has been saved once. If it has, then it is saved
    /// again to the same path. If not, calls [Gui::get_output()] to allow the
    /// user to select a path to save to.
    fn save_file(&self) {
        let filename: String = if *self.saved_once.borrow() {
            self.filename.borrow().to_string()
        } else {
            match self.get_output() {
                Some(c) => {
                    self.saved_once.swap(&RefCell::new(true));
                    self.filename.swap(&RefCell::new(c.to_string()));
                    c
                }
                None => return,
            }
        };
        if *self.saved_once.borrow() {
            self.get_specs(&filename).run();
            self.save_template(&filename);
            self.saved_current.swap(&RefCell::new(true));
            self.set_window_title();
        }
    }

    /// Saves file under a new name whether it has already been saved or not.
    fn save_file_as(&self) {
        if let Some(c) = self.get_output() {
            self.saved_once.swap(&RefCell::new(true));
            self.filename.swap(&RefCell::new(c.to_string()));
            self.get_specs(&c).run();
            self.save_template(&c);
            self.saved_current.swap(&RefCell::new(true));
            self.set_window_title();
        };
    }

    fn open_template(&self) {
        let dialog = gtk::FileChooserDialog::with_buttons::<Window>(
            Some("Open Template"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
            &[
                ("_Cancel", ResponseType::Cancel),
                ("_Ok", ResponseType::Accept),
            ],
        );
        let filter = gtk::FileFilter::new();
        filter.add_pattern("*.toml");
        filter.set_name(Some("toml files"));
        dialog.add_filter(&filter);
        let res = dialog.run();
        let filename: Option<String> = if res == Accept {
            dialog
                .get_filename().and_then(|name| match name.to_str() {
                    Some(c) => Some(c.to_string()),
                    None => None,
                })
        } else {
            None
        };
        dialog.close();
        match filename {
            Some(t) => {
                if let Some(template) = Template::load_from_file(PathBuf::from(t)) {
                    self.load_template(&template);
                }
            }
            None => println!("Nothing selected"),
        };
    }

    /// Saves a template (toml format) to the specified location
    fn save_template(&self, file: &str) {
        let data: Template = self.template_from_gui();
        data.save_to_file(&PathBuf::from(file));
    }

    /// Updates the title of the program window with the name of the output file.
    fn set_window_title(&self) {
        if !*self.saved_once.borrow() {
            self.window
                .set_title(&format!("Gfret - {} - <unsaved>", crate_version!()));
        } else if *self.saved_current.borrow() {
            self.window.set_title(&format!(
                "Gfret - {} - {}",
                crate_version!(),
                self.filename.borrow().split('/').last().unwrap()
            ));
        } else {
            self.window.set_title(&format!(
                "Gfret - {} - {}*",
                crate_version!(),
                self.filename.borrow().split('/').last().unwrap()
            ));
        }
    }

    /// Saves the program state before exiting
    fn cleanup(&self) {
        let data: Template = self.template_from_gui();
        data.save_statefile();
    }

    /// Loads styling from css before launching the gui
    fn init_css() {
        let config = match Config::from_file() {
            Some(c) => c,
            None => Config::new(),
        };
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(
                format!("spinbutton button {{ min-height: 0; min-width: 0; padding: 1px; }}
                    viewport {{ background-color: {}; }}", config.background_color)
                    .as_bytes(),
            )
            .expect("Failed to load CSS");

        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

pub fn run_ui(template: Option<&str>) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let gui = Rc::new(Gui::new());

    if let Some(t) = template {
        let path = PathBuf::from(t);
        if path.exists() {
            if let Some(template) = Template::load_from_file(path) {
                gui.load_template(&template);
            }
        }
    } else {
        let mut statefile = CONFIGDIR.clone();
        statefile.push("state.toml");
        if statefile.exists() {
            if let Some(template) = Template::load_from_file(statefile) {
                gui.load_template(&template);
            }
        }
    }
    Gui::init_css();

    gui.window
        .set_title(&format!("Gfret - {} - <unsaved>", crate_version!()));
    gui.draw_preview(false);

    gui.checkbox_multi
        .connect_toggled(clone!(@weak gui => move |_| {
            gui.toggle_multi();
            gui.draw_preview(true);
        }));

    gui.scale
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.scale_multi_course
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.fret_count
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.perpendicular_fret
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.nut_width
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.bridge_spacing
        .connect_value_changed(clone!(@weak gui => move |_| {
            gui.draw_preview(true);
        }));

    gui.window
        .connect_check_resize(clone!(@weak gui => move |_| {
            gui.draw_preview(false);
        }));

    gui.save_file
        .connect_activate(clone!(@weak gui => move |_| {
            gui.save_file();
        }));

    gui.save_as.connect_activate(clone!(@weak gui => move |_| {
        gui.save_file_as();
    }));

    gui.open_template
        .connect_activate(clone!(@weak gui => move |_| {
            gui.open_template();
        }));

    gui.open_external
        .connect_activate(clone!(@weak gui => move |_| {
            gui.open_external();
        }));

    gui.preferences.connect_activate(|_| {
        prefs::run();
    });

    gui.quit.connect_activate(clone!(@weak gui => move |_| {
        gui.cleanup();
        gtk::main_quit();
    }));

    let clone = gui.clone();
    gui.window.connect_delete_event(move |_, _| {
        clone.cleanup();
        gtk::main_quit();
        Inhibit(false)
    });

    gui.window.show_now();
    gtk::main()
}
