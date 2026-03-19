use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Widget};
use std::rc::Rc;
use std::cell::RefCell;

use crate::calendar;

pub fn set_input(page: Rc<RefCell<calendar::Page>>) {
    let controller = gtk::EventControllerKey::new();
    let value = page.clone();
    controller.connect_key_pressed(move |_, key, _key_code, modifier_type| {
        if key == gtk::gdk::Key::Escape {
            std::process::exit(0);
        }
        if key == gtk::gdk::Key::L || (key == gtk::gdk::Key::l && modifier_type == gtk::gdk::ModifierType::CONTROL_MASK) {
            value.borrow_mut().current_month += 1;
            value.borrow().make_page();
        }
        if key == gtk::gdk::Key::H || (key == gtk::gdk::Key::h && modifier_type == gtk::gdk::ModifierType::CONTROL_MASK) {
            value.borrow_mut().current_month -= 1;
            value.borrow().make_page();
        }
        if key == gtk::gdk::Key::h {
            value.borrow().window.emit_move_focus(gtk4::DirectionType::Left);
        }
        if key == gtk::gdk::Key::j {
            value.borrow().window.emit_move_focus(gtk4::DirectionType::Down);
        }
        if key == gtk::gdk::Key::k {
            value.borrow().window.emit_move_focus(gtk4::DirectionType::Up);
        }
        if key == gtk::gdk::Key::l {
            value.borrow().window.emit_move_focus(gtk4::DirectionType::Right);
        }

        if key == gtk::gdk::Key::r {
            value.borrow_mut().reset_current_month();
            value.borrow().make_page();
        }
        glib::Propagation::Proceed
    });
    page.borrow().window.add_controller(controller);
}
