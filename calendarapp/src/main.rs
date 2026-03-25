use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

use std::rc::Rc;
use std::cell::RefCell;

mod input;
mod layout;
mod calendar;
// mod notes;

// debug
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Result};
// debug

fn run_app(app: &Application) {

    layout::set_style();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Calendar")
        .default_width(350)
        .default_height(450)
        .modal(true)
        .decorated(false)
        .build();


    let current_page = Rc::new(RefCell::new(calendar::Page::new(window.clone())));
    current_page.borrow_mut().make_page();
    input::set_input(current_page);

    window.present();
} 


fn main() -> glib::ExitCode {
    // let notes = vec!(
    //     notes::Note::new(Local::now(), "note1", "this is my first note."),
    //     notes::Note::new(Local::now(), "note2", "this is my second note.")
    // );
    //
    // // let j = serde_json::to_string_pretty(&notes);
    //
    // notes::read_notes(Local::now());





    let application = Application::builder()
        .application_id("com.example.CalendarApp")
        .build();

    application.connect_activate(run_app);
    application.run()
}
