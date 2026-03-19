use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{CssProvider, Button, Grid, Label};
use chrono::prelude::*;
use chrono::{Duration};

use crate::calendar;

pub fn set_style() {
    let css = "
        window { 
        color: #DDDD88;
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

impl calendar::Page {

    pub fn make_page(&self) {

        // let mut current_date = Utc.with_ymd_and_hms(self.date.year(), (self.date.month() as i32 + self.current_month) as u32, 1, 12, 0, 0).unwrap();
        // let mut current_date = Utc.with_ymd_and_hms(self.date.year(), self.date.month(), 1, 12, 0, 0).unwrap();
        let mut current_date = Local.with_ymd_and_hms(self.date.year(), self.date.month(), 1, 12, 0, 0).unwrap();
        calendar::increment_month(&mut current_date, self.current_month);


        // build layout
        let grid = Grid::builder()
            .column_homogeneous(true)
            .row_homogeneous(true)
            .build();

        let title = &Label::new(Some(&format!("{}, {}", current_date.format("%B"), current_date.year())));
        grid.attach(title, 0, 0, 7, 1);

        let weekday_labels = vec![
            Label::new(Some("Mon")),
            Label::new(Some("Tue")),
            Label::new(Some("Wed")),
            Label::new(Some("Thu")),
            Label::new(Some("Fri")),
            Label::new(Some("Sat")),
            Label::new(Some("Sun"))
        ];

        for i in 0..7 {
            grid.attach(&weekday_labels[i], i as i32, 1, 1, 1);
        }

        let current_month = current_date.month();
        let mut current_week: i32 = 2;

        // previous month fill
        current_date -= Duration::days(current_date.weekday().num_days_from_monday() as i64);
        while current_date.month() != current_month {
            // TODO: Add style and button funcitons
            let button = Button::with_label(&current_date.day().to_string());
            grid.attach(
                &button,
                current_date.weekday().num_days_from_monday().try_into().unwrap(),
                current_week,
                1,
                1
            );
            current_date += Duration::days(1);
        }

        while current_date.month() == current_month {
            // TODO: Add style and button funcitons
            let button = Button::with_label(&current_date.day().to_string());
            grid.attach(
                &button,
                current_date.weekday().num_days_from_monday().try_into().unwrap(),
                current_week,
                1,
                1
            );

            if current_date.weekday() == Weekday::Sun {
                current_week += 1;
            }
            current_date += Duration::days(1);
        }

        // next month fill
        while current_date.weekday().num_days_from_monday() > 0 {
            // TODO: Add style and button funcitons
            let button = Button::with_label(&current_date.day().to_string());
            grid.attach(
                &button,
                current_date.weekday().num_days_from_monday().try_into().unwrap(),
                current_week,
                1,
                1
            );
            current_date += Duration::days(1);
        }

        // println!("{}", current_date);

        self.window.set_child(Some(&grid));
    }
}

