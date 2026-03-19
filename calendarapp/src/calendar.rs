use gtk4 as gtk;
use gtk::{ApplicationWindow};
use chrono::prelude::*;
use chrono::{ Duration, DateTime};


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

    // pub fn reset_current_month(&mut self) {
    //     self.current_month = 0;
    // }
}

pub fn increment_month(date: &mut DateTime<Local>, mut months: i32) {
    let mut current_month = date.month();

    while months > 0 {
        while date.month() == current_month {
            *date += Duration::days(1);
        }
        current_month = date.month();
        months -= 1;
    }

    while months < 0 {
        while date.month() == current_month {
            *date -= Duration::days(1);
        }
        current_month = date.month();
        months += 1;
    }

    *date = date.with_day(1).unwrap();
}
