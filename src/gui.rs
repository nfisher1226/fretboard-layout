#![warn(clippy::all, clippy::pedantic)]
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
use crate::gdk_pixbuf::Pixbuf;
use crate::gio::AppInfoExt;
use crate::gio::Cancellable;
use crate::gio::MemoryInputStream;
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
    external_program: gtk::AppChooserButton,
    saved: gtk::CheckButton,
    filename: gtk::Entry,
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

    fn open_external(&self, specs: &Specs) {
        let cmd = self.get_cmd();
        Command::new(cmd).args(&[&specs.output]).spawn().unwrap();
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
        external_program: builder.get_object("external_program").unwrap(),
        saved: builder.get_object("saved").unwrap(),
        filename: builder.get_object("filename").unwrap(),
    });

    let window0 = Rc::new(window);
    let window1 = window0.clone();
    let window_size = window1.get_size();
    let widgets0 = Rc::new(widgets);
    widgets0.draw_preview(window_size.0);

    let widgets1 = widgets0.clone();
    let widgets2 = widgets0.clone();
    widgets1
        .checkbox_multi
        .connect_toggled(move |_| widgets2.toggle_multi());

    let widgets5 = widgets0.clone();
    let widgets6 = widgets0.clone();
    let window2 = window0.clone();
    widgets5.scale.connect_value_changed(move |_| {
        let window_size = window2.get_size();
        widgets6.draw_preview(window_size.0);
    });

    let widgets7 = widgets0.clone();
    let widgets8 = widgets0.clone();
    let window3 = window0.clone();
    widgets7.checkbox_multi.connect_toggled(move |_| {
        let window_size = window3.get_size();
        widgets8.draw_preview(window_size.0);
    });

    let widgets9 = widgets0.clone();
    let widgets10 = widgets0.clone();
    let window4 = window0.clone();
    widgets9.scale_multi_course.connect_value_changed(move |_| {
        let window_size = window4.get_size();
        widgets10.draw_preview(window_size.0);
    });

    let widgets11 = widgets0.clone();
    let widgets12 = widgets0.clone();
    let window5 = window0.clone();
    widgets11.fret_count.connect_value_changed(move |_| {
        let window_size = window5.get_size();
        widgets12.draw_preview(window_size.0);
    });

    let widgets13 = widgets0.clone();
    let widgets14 = widgets0.clone();
    let window6 = window0.clone();
    widgets13
        .perpendicular_fret
        .connect_value_changed(move |_| {
            let window_size = window6.get_size();
            widgets14.draw_preview(window_size.0);
        });

    let widgets15 = widgets0.clone();
    let widgets16 = widgets0.clone();
    let window7 = window0.clone();
    widgets15.nut_width.connect_value_changed(move |_| {
        let window_size = window7.get_size();
        widgets16.draw_preview(window_size.0);
    });

    let widgets17 = widgets0.clone();
    let widgets18 = widgets0.clone();
    let window8 = window0.clone();
    widgets17.bridge_spacing.connect_value_changed(move |_| {
        let window_size = window8.get_size();
        widgets18.draw_preview(window_size.0);
    });

    let widgets19 = widgets0.clone();
    let widgets20 = widgets0.clone();
    let window9 = window0.clone();
    widgets19.border.connect_value_changed(move |_| {
        let window_size = window9.get_size();
        widgets20.draw_preview(window_size.0);
    });

    let widgets21 = widgets0.clone();
    let window10 = window0.clone();
    let window11 = window0.clone();
    window10.connect_check_resize(move |_| {
        let window_size = window11.get_size();
        widgets21.draw_preview(window_size.0);
    });

    let save_button: gtk::ToolButton = builder.get_object("save_button").unwrap();
    let widgets22 = widgets0.clone();
    save_button.connect_clicked(move |_| {
        let filename: String = if widgets22.saved.get_active() {
            widgets22.filename.get_text().to_string()
        } else {
            match widgets22.get_output() {
                Some(c) => {
                    widgets22.saved.set_active(true);
                    widgets22.filename.set_text(&c);
                    c
                },
                _ => "".to_string(),
            }
        };
        if widgets22.saved.get_active() {
            widgets22.get_specs(&filename).run();
        }
    });

    let close_button: gtk::ToolButton = builder.get_object("quit_button").unwrap();
    close_button.connect_clicked(|_| gtk::main_quit());

    let window20 = window0.clone();
    window20.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window0.show_now();

    gtk::main()
}
