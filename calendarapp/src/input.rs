use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{ glib };
use gtk::gdk::{ Key, ModifierType };
use std::rc::Rc;
use std::cell::RefCell;

use crate::calendar;

pub fn set_input(page: Rc<RefCell<calendar::Page>>) {
    let controller = gtk::EventControllerKey::new();
    let value = page.clone();
    controller.connect_key_pressed(move |_, key, _key_code, modifier_type| {

        match(key, modifier_type) {
            (Key::Escape, _) => std::process::exit(0),

            (Key::L, _) | (Key::l, ModifierType::CONTROL_MASK) => {
                value.borrow_mut().current_month += 1;
                value.borrow().make_page();
            }
            (Key::H, _) | (Key::h, ModifierType::CONTROL_MASK) => {
                value.borrow_mut().current_month -= 1;
                value.borrow().make_page();
            }
            (Key::K, _) | (Key::k, ModifierType::CONTROL_MASK) => {
                value.borrow_mut().current_month += 12; 
                value.borrow().make_page();
            }
            (Key::J, _) | (Key::j, ModifierType::CONTROL_MASK) => {
                value.borrow_mut().current_month -= 12; 
                value.borrow().make_page();
            }

            (Key::h, _) => value.borrow().window.emit_move_focus(gtk4::DirectionType::Left),
            (Key::j, _) => value.borrow().window.emit_move_focus(gtk4::DirectionType::Down),
            (Key::k, _) => value.borrow().window.emit_move_focus(gtk4::DirectionType::Up),
            (Key::l, _) => value.borrow().window.emit_move_focus(gtk4::DirectionType::Right),

            (Key::r, _) => {
                value.borrow_mut().reset_current_month();
                value.borrow().make_page();
            }

            (_, _) => {}
        }

        glib::Propagation::Proceed
    });
    page.borrow().window.add_controller(controller);
}
