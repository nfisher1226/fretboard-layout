#![warn(clippy::all, clippy::pedantic)]
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
use crate::crate_version;
use crate::gdk_pixbuf::Pixbuf;
use crate::gio::AppInfoExt;
use crate::gio::Cancellable;
use crate::gio::MemoryInputStream;
use crate::glib::clone;
use crate::gtk::prelude::*;
use crate::gtk::{
    DialogExt, EntryExt, FileChooserAction, FileChooserExt, Inhibit, RangeExt, ResponseType,
    SpinButtonExt, ToggleButtonExt, WidgetExt, Window, WindowType,
};
use crate::Specs;

use gtk::ResponseType::Accept;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::cell::RefCell;

struct Widgets {
    image_preview: gtk::Image,
    scale: gtk::Scale,
    checkbox_multi: gtk::CheckButton,
    scale_multi_course: gtk::Scale,
    scale_multi_fine: gtk::SpinButton,
    fret_count: gtk::SpinButton,
    perpendicular_fret: gtk::SpinButton,
    nut_width: gtk::SpinButton,
    bridge_spacing: gtk::SpinButton,
    border: gtk::SpinButton,
    external_button: gtk::ToolButton,
    external_program: gtk::AppChooserButton,
    saved_once: Rc<RefCell<bool>>,
    saved_current: Rc<RefCell<bool>>,
    filename: gtk::Entry,
    save_button: gtk::ToolButton,
    save_as_button: gtk::ToolButton,
    quit_button: gtk::ToolButton,
}

impl Widgets {
    fn get_specs(&self, filename: &str) -> Specs {
        Specs {
            scale: self.scale.get_value(),
            count: self.fret_count.get_value() as u32,
            multi: self.checkbox_multi.get_active(),
            scale_treble: self.scale_multi_course.get_value(),
            nut: self.nut_width.get_value(),
            bridge: self.bridge_spacing.get_value(),
            pfret: self.perpendicular_fret.get_value(),
            output: filename.to_string(),
            border: self.border.get_value(),
            external: false,
            cmd: self.get_cmd(),
        }
    }

    fn draw_preview(&self, width: i32) {
        let image = self.get_specs("-").create_document().to_string();
        let bytes = glib::Bytes::from_owned(image.into_bytes());
        let stream = gio::MemoryInputStream::from_bytes(&bytes);
        let pixbuf = Pixbuf::from_stream_at_scale::<MemoryInputStream, Cancellable>(
            &stream, width, -1, true, None,
        );
        self.image_preview.set_from_pixbuf(Some(&pixbuf.unwrap()));
    }

    fn toggle_multi(&self) {
        let value = self.checkbox_multi.get_active();
        self.scale_multi_course.set_sensitive(value);
        self.scale_multi_fine.set_sensitive(value);
        self.perpendicular_fret.set_sensitive(value);
    }

    fn get_cmd(&self) -> String {
        let cmd = self.external_program.get_app_info();
        let cmd = match cmd {
            Some(c) => c.get_commandline(),
            None => Some(PathBuf::from("xdg-open")),
        };
        match cmd {
            Some(c) => c
                .into_os_string()
                .into_string()
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string(),
            None => "xdg-open".to_string(),
        }
    }

    fn open_external(&self) {
        if !*self.saved_current.borrow() {
            self.save_button.emit_clicked();
        }
        if *self.saved_current.borrow() {
            let cmd = self.get_cmd();
            let filename = self.filename.get_text().to_string();
            Command::new(cmd).args(&[&filename]).spawn().unwrap();
        }
    }

    fn get_output(&self) -> Option<String> {
        let currentfile = if *self.saved_once.borrow() {
            self.filename.get_text().to_string()
        } else {
            String::from("unitled.svg")
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
            match dialog.get_filename().unwrap().to_str() {
                Some(c) => Some(c.to_string()),
                None => Some(currentfile),
            }
        } else {
            None
        };
        dialog.close();
        filename
    }

    fn save_file(&self) {
        let filename: String = if *self.saved_once.borrow() {
            self.filename.get_text().to_string()
        } else {
            match self.get_output() {
                Some(c) => {
                    self.saved_once.swap(&RefCell::new(true));
                    self.filename.set_text(&c);
                    c
                }
                None => "".to_string(),
            }
        };
        if *self.saved_once.borrow() {
            self.get_specs(&filename).run();
            self.saved_current.swap(&RefCell::new(true));
        }
    }

    fn save_file_as(&self) {
        match self.get_output() {
            Some(c) => {
                self.saved_once.swap(&RefCell::new(true));
                self.filename.set_text(&c);
                self.get_specs(&c).run();
                self.saved_current.swap(&RefCell::new(true));
            }
            None => return,
        };
    }

    fn set_window_title(&self, window: Rc<gtk::Window>) {
        if ! *self.saved_once.borrow() {
            window.set_title(&format!("Gfret - {} - <unsaved>", crate_version!()));
        } else if *self.saved_current.borrow() {
            window.set_title(&format!("Gfret - {} - {}", crate_version!(), self.filename.get_text().split("/").last().unwrap()));
        } else {
            window.set_title(&format!("Gfret - {} - {}*", crate_version!(), self.filename.get_text().split("/").last().unwrap()));
        }
    }
}

pub fn run_gui() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("ui.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("mainWindow").unwrap();
    window.set_title(&format!("Gfret - {} - <unsaved>", crate_version!()));

    let widgets = Rc::new(Widgets {
        image_preview: builder.get_object("image_preview").unwrap(),
        scale: builder.get_object("scale_course").unwrap(),
        checkbox_multi: builder.get_object("check_box_multi").unwrap(),
        scale_multi_course: builder.get_object("scale_multi_course").unwrap(),
        scale_multi_fine: builder.get_object("scale_multi_fine").unwrap(),
        fret_count: builder.get_object("fret_count").unwrap(),
        perpendicular_fret: builder.get_object("perpendicular_fret").unwrap(),
        nut_width: builder.get_object("nut_width").unwrap(),
        bridge_spacing: builder.get_object("bridge_spacing").unwrap(),
        border: builder.get_object("border").unwrap(),
        external_button: builder.get_object("external_button").unwrap(),
        external_program: builder.get_object("external_program").unwrap(),
        saved_once: Rc::new(RefCell::new(false)),
        saved_current: Rc::new(RefCell::new(false)),
        filename: builder.get_object("filename").unwrap(),
        save_button: builder.get_object("save_button").unwrap(),
        save_as_button: builder.get_object("save_as_button").unwrap(),
        quit_button: builder.get_object("quit_button").unwrap(),
    });

    let window = Rc::new(window);
    let window_size = window.get_size();
    let widgets = Rc::new(widgets);
    widgets.draw_preview(window_size.0);

    widgets
        .checkbox_multi
        .connect_toggled(clone!(@weak widgets, @weak window => move |_| {
                widgets.toggle_multi();
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
                widgets.saved_current.swap(&RefCell::new(false));
                widgets.set_window_title(window);
        }));

    widgets
        .scale
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
                widgets.saved_current.swap(&RefCell::new(false));
                widgets.set_window_title(window);
        }));

    widgets
        .scale_multi_course
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
                widgets.saved_current.swap(&RefCell::new(false));
                widgets.set_window_title(window);
        }));

    widgets
        .fret_count
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
                widgets.saved_current.swap(&RefCell::new(false));
                widgets.set_window_title(window);
        }));

    widgets
        .perpendicular_fret
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
                widgets.saved_current.swap(&RefCell::new(false));
                widgets.set_window_title(window);
        }));

    widgets
        .nut_width
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
                widgets.saved_current.swap(&RefCell::new(false));
                widgets.set_window_title(window);
        }));

    widgets
        .bridge_spacing
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
                widgets.saved_current.swap(&RefCell::new(false));
                widgets.set_window_title(window);
        }));

    widgets
        .border
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
                widgets.saved_current.swap(&RefCell::new(false));
                widgets.set_window_title(window);
        }));

    window.connect_check_resize(clone!(@weak window, @weak widgets => move |_| {
        let window_size = window.get_size();
        widgets.draw_preview(window_size.0);
    }));

    widgets
        .save_button
        .connect_clicked(clone!(@weak widgets, @weak window => move |_| {
            widgets.save_file();
            widgets.set_window_title(window);
        }));

    widgets
        .save_as_button
        .connect_clicked(clone!(@weak widgets, @weak window => move |_| {
            widgets.save_file_as();
            widgets.set_window_title(window);
        }));

    widgets
        .external_button
        .connect_clicked(clone!(@weak widgets => move |_| widgets.open_external()));

    widgets.quit_button.connect_clicked(|_| gtk::main_quit());

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.show_now();

    gtk::main()
}
