use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib};
use std::rc::Rc;
use std::cell::RefCell;

use crate::calendar;

pub fn set_input(page: Rc<RefCell<calendar::Page>>) {
    let controller = gtk::EventControllerKey::new();
    let value = page.clone();
    controller.connect_key_pressed(move |_, key, _key_code, _modifier_type| {
        if key == gtk::gdk::Key::Escape {
            std::process::exit(0);
        }
        if key == gtk::gdk::Key::Right {
            value.borrow_mut().current_month += 1;
        }
        if key == gtk::gdk::Key::Left {
            value.borrow_mut().current_month -= 1;
        }
        value.borrow().make_page();
        glib::Propagation::Proceed
    });
    page.borrow().window.add_controller(controller);
}
