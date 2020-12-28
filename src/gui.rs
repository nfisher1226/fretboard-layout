#![warn(clippy::all, clippy::pedantic)]
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
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
    saved_once: gtk::CheckButton,
    saved_current: gtk::CheckButton,
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
        self.saved_current.set_active(false);
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
            _ => Some(PathBuf::from("xdg-open")),
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
            _ => "xdg-open".to_string(),
        }
    }

    fn open_external(&self) {
        if !self.saved_current.get_active() {
            self.save_button.emit_clicked();
        }
        if self.saved_current.get_active() {
            let cmd = self.get_cmd();
            let filename = self.filename.get_text().to_string();
            Command::new(cmd).args(&[&filename]).spawn().unwrap();
        }
    }

    fn get_output(&self) -> Option<String> {
        let currentfile = self.filename.get_text();
        let dialog = gtk::FileChooserDialog::with_buttons::<Window>(
            Some("Save As"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
            &[
                ("_Cancel", ResponseType::Cancel),
                ("_Ok", ResponseType::Accept),
            ],
        );
        dialog.set_current_name(currentfile.to_string());
        dialog.set_do_overwrite_confirmation(true);
        let res = dialog.run();
        let filename: Option<String> = if res == Accept {
            match dialog.get_filename().unwrap().to_str() {
                Some(c) => Some(c.to_string()),
                _ => Some(currentfile.to_string()),
            }
        } else {
            None
        };
        dialog.close();
        filename
    }

    fn save_file(&self) {
        let filename: String = if self.saved_once.get_active() {
            self.filename.get_text().to_string()
        } else {
            match self.get_output() {
                Some(c) => {
                    self.saved_once.set_active(true);
                    self.filename.set_text(&c);
                    c
                }
                _ => "".to_string(),
            }
        };
        if self.saved_once.get_active() {
            self.get_specs(&filename).run();
            self.saved_current.set_active(true);
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
        saved_once: builder.get_object("saved_once").unwrap(),
        saved_current: builder.get_object("saved_current").unwrap(),
        filename: builder.get_object("filename").unwrap(),
        save_button: builder.get_object("save_button").unwrap(),
        save_as_button: builder.get_object("save_as_button").unwrap(),
        quit_button: builder.get_object("quit_button").unwrap(),
    });

    let window = Rc::new(window);
    let window_size = window.clone().get_size();
    let widgets = Rc::new(widgets);
    widgets.draw_preview(window_size.0);

    widgets
        .clone()
        .checkbox_multi
        .connect_toggled(clone!(@weak widgets, @weak window => move |_| {
                widgets.toggle_multi();
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
        }));

    widgets
        .clone()
        .scale
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
        }));

    widgets
        .clone()
        .scale_multi_course
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
        }));

    widgets
        .clone()
        .fret_count
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
        }));

    widgets
        .clone()
        .perpendicular_fret
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
        }));

    widgets
        .clone()
        .nut_width
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
        }));

    widgets
        .clone()
        .bridge_spacing
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
        }));

    widgets
        .clone()
        .border
        .connect_value_changed(clone!(@weak widgets, @weak window => move |_| {
                let window_size = window.get_size();
                widgets.draw_preview(window_size.0);
        }));

    window.clone().connect_check_resize(clone!(@weak window, @weak widgets => move |_| {
        let window_size = window.get_size();
        widgets.draw_preview(window_size.0);
    }));

    widgets
        .clone()
        .save_button
        .connect_clicked(clone!(@weak widgets => move |_| widgets.save_file()));

    widgets
        .clone()
        .external_button
        .connect_clicked(clone!(@weak widgets => move |_| widgets.open_external()));

    widgets.clone().quit_button.connect_clicked(|_| gtk::main_quit());

    window.clone().connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window.show_now();

    gtk::main()
}