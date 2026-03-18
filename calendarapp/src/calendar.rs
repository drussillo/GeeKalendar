use gtk4 as gtk;
use gtk::{ApplicationWindow};
use chrono::prelude::*;


pub struct Page {
    pub window: ApplicationWindow,
    pub date: DateTime<Local>,
    pub current_month: i32
}

impl Page {
    pub fn new(window: ApplicationWindow) -> Self {
        Self { 
            window,
            date: Local::now(),
            current_month: 0
        }
    }
}

