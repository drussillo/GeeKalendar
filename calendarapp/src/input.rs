use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, ApplicationWindow};

pub fn set_input(window: &ApplicationWindow) {
    let controller = gtk::EventControllerKey::new();
    controller.connect_key_pressed(move |_, key, _key_code, _modifier_type| {
        if key == gtk::gdk::Key::Escape {
            std::process::exit(0);
        }
        glib::Propagation::Proceed
    });
    window.add_controller(controller);
}
