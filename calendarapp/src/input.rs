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
        // if key == gtk::gdk::Key::Right && modifier_type == gtk::gdk::ModifierType::CONTROL_MASK {
        if key == gtk::gdk::Key::L {
            value.borrow_mut().current_month += 1;
            value.borrow().make_page();
        }
        // if key == gtk::gdk::Key::Left && modifier_type == gtk::gdk::ModifierType::CONTROL_MASK {
        if key == gtk::gdk::Key::H {
            value.borrow_mut().current_month -= 1;
            value.borrow().make_page();
        }
        if key == gtk::gdk::Key::r {
            value.borrow_mut().reset_current_month();
            value.borrow().make_page();
        }
        glib::Propagation::Proceed
    });
    page.borrow().window.add_controller(controller);
}
