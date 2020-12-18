#![warn(clippy::all, clippy::pedantic)]
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
use crate::gdk_pixbuf::Pixbuf;
use crate::gio::Cancellable;
use crate::gio::MemoryInputStream;
use crate::gtk::prelude::*;
use crate::gtk::{
    ButtonExt, EntryExt, Inhibit, RangeExt, SpinButtonExt, ToggleButtonExt, WidgetExt,
};
use crate::Specs;

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
    output: gtk::Entry,
    checkbox_extern: gtk::CheckButton,
    external: gtk::Entry,
}

impl Widgets {
    fn get_specs(&self) -> Specs {
        Specs {
            scale: self.scale.get_value(),
            count: self.fret_count.get_value() as u32,
            multi: self.checkbox_multi.get_active(),
            scale_treble: self.scale_multi_course.get_value(),
            nut: self.nut_width.get_value(),
            bridge: self.bridge_spacing.get_value(),
            pfret: self.perpendicular_fret.get_value(),
            output: self.output.get_text().to_string(),
            border: self.border.get_value(),
            external: self.checkbox_extern.get_active(),
            cmd: self.external.get_text().to_string(),
        }
    }

    fn draw_preview(&self, width: i32) {
        let image = self.get_specs().create_document().to_string();
        let bytes = glib::Bytes::from_owned(image.into_bytes());
        let stream = gio::MemoryInputStream::from_bytes(&bytes);
        let pixbuf = Pixbuf::from_stream_at_scale::<MemoryInputStream, Cancellable>(
            &stream, width, -1, false, None,
        );
        self.image_preview.set_from_pixbuf(Some(&pixbuf.unwrap()));
    }

    fn toggle_multi(&self) {
        let value = self.checkbox_multi.get_active();
        self.scale_multi_course.set_sensitive(value);
        self.scale_multi_fine.set_sensitive(value);
        self.perpendicular_fret.set_sensitive(value);
    }

    fn toggle_extern(&self) {
        self.external.set_sensitive(self.checkbox_extern.get_active());
    }
}

pub fn run_gui() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("gfret_ui.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("mainWindow").unwrap();

    let widgets = Rc::new(Widgets {
        image_preview: builder.get_object("imagePreview").unwrap(),
        scale: builder.get_object("scaleCourse").unwrap(),
        checkbox_multi: builder.get_object("checkBoxMulti").unwrap(),
        scale_multi_course: builder.get_object("scaleMultiCourse").unwrap(),
        scale_multi_fine: builder.get_object("scaleMultiFine").unwrap(),
        fret_count: builder.get_object("fretCount").unwrap(),
        perpendicular_fret: builder.get_object("perpendicularFret").unwrap(),
        nut_width: builder.get_object("nutWidth").unwrap(),
        bridge_spacing: builder.get_object("bridgeSpacing").unwrap(),
        border: builder.get_object("border").unwrap(),
        output: builder.get_object("output").unwrap(),
        checkbox_extern: builder.get_object("checkBoxExtern").unwrap(),
        external: builder.get_object("external").unwrap(),
    });

    let window0 = Rc::new(window);
    let window1 = window0.clone();
    let window_size = window1.get_size();
    let widgets0 = Rc::new(widgets);
    widgets0.draw_preview(window_size.0);

    let widgets1 = widgets0.clone();
    let widgets2 = widgets0.clone();
    widgets1.checkbox_multi.connect_toggled(move |_| widgets2.toggle_multi());

    let widgets3 = widgets0.clone();
    let widgets4 = widgets0.clone();
    widgets3.checkbox_extern.connect_toggled(move |_| widgets4.toggle_extern());

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
    widgets13.perpendicular_fret.connect_value_changed(move |_| {
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

    let save_button: gtk::Button = builder.get_object("saveButton").unwrap();
    let widgets21 = widgets0.clone();
    save_button.connect_clicked(move |_| widgets21.get_specs().run());

    let close_button: gtk::Button = builder.get_object("closeButton").unwrap();
    close_button.connect_clicked(|_| gtk::main_quit());

    let window20 = window0.clone();
    window20.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window0.show_all();

    gtk::main()
}
