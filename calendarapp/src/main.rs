use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

mod input;
mod layout;
mod calendar;

fn run_app(app: &Application) {

    layout::set_style();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Calendar")
        .default_width(350)
        .default_height(350)
        .modal(true)
        .decorated(false)
        .build();

    input::set_input(&window);
    layout::make_layout(&window);
    // layout::add_button(&window);

    window.present();
} 


fn main() -> glib::ExitCode {
    calendar::get_date_test();

    let application = Application::builder()
        .application_id("com.example.CalendarApp")
        .build();

    application.connect_activate(run_app);

    application.run()
}
