use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, CssProvider};

mod input;

fn run_app(app: &Application) {
    let css = "
        window { 
        color: #FF7777;
        background-color: #181818; 
        font-size: 18px; 
        font-family: \"UbuntuMono Nerd Font\"; 
        }";
    let provider = CssProvider::new();
    provider.load_from_data(css);

    let display = gtk::gdk::Display::default().expect("No display");
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Calendar")
        .default_width(350)
        .default_height(350)
        .modal(true)
        .decorated(false)
        .build();

    let button = Button::with_label("Click me!");
    button.connect_clicked(|_| {
        eprintln!("Clicked!");
    });
    window.set_child(Some(&button));

    window.present();
} 


fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.CalendarApp")
        .build();

    application.connect_activate(run_app);

    application.run()
}
