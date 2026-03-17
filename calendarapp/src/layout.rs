use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{CssProvider, ApplicationWindow, Button};

pub fn set_style() {
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
}

pub fn add_button(window: &ApplicationWindow) {
    let button = Button::with_label("Click me!");
    button.connect_clicked(|_| {
        eprintln!("Clicked!");
    });
    window.set_child(Some(&button));
}
