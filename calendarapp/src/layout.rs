use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{CssProvider, Button, Grid, Label};
use chrono::prelude::*;
use chrono::{Duration};

use crate::calendar;

pub fn set_style() {
    let provider = CssProvider::new();
    provider.load_from_path("/home/dave/code/CalendarApp/calendarapp/resources/style.css");  // TODO resource path

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
        let page_grid = Grid::builder()
            .row_spacing(1)
            .column_spacing(1)
            .column_homogeneous(true)
            .row_homogeneous(true)
            .build();

        let title = &Label::new(Some(&format!("{}, {}", current_date.format("%B"), current_date.year())));
        title.add_css_class("title");
        page_grid.attach(title, 0, 0, 7, 1);

        let weekday_labels = vec![
            Label::new(Some("Mon")),
            Label::new(Some("Tue")),
            Label::new(Some("Wed")),
            Label::new(Some("Thu")),
            Label::new(Some("Fri")),
            Label::new(Some("Sat")),
            Label::new(Some("Sun"))
        ];

        // add style
        for label in &weekday_labels {
            label.add_css_class("weekday")
        }

        if self.start_sun {
            for i in 1..7 {
                page_grid.attach(&weekday_labels[i-1], i as i32, 1, 1, 1);
            }
            page_grid.attach(&weekday_labels[6], 0, 1, 1, 1);
        } else {
            for i in 0..7 {
                page_grid.attach(&weekday_labels[i], i as i32, 1, 1, 1);
            }
        }

        let days_grid = Grid::builder()
            .row_spacing(1)
            .column_spacing(1)
            .column_homogeneous(true)
            .row_homogeneous(true)
            .build();

        page_grid.attach(&days_grid, 0, 2, 7, 5);

        let current_month = current_date.month();
        let mut current_week: i32 = 0;

        // previous month fill
        current_date -= Duration::days(calendar::days_from_start(&current_date, self.start_sun) as i64);
        while current_date.month() != current_month {
            let button = Button::with_label(&current_date.day().to_string());
            // TODO: Check for notes and update style
            button.set_sensitive(false);
            button.add_css_class("disabled-day");
            days_grid.attach(
                &button,
                calendar::days_from_start(&current_date, self.start_sun).try_into().unwrap(),
                current_week,
                1,
                1
            );
            current_date += Duration::days(1);
        }

        // current month fill
        let mut today_row = 0;
        let mut today_col = 0;
        while current_date.month() == current_month {
            // TODO: Add button funcitons
            let button = Button::with_label(&current_date.day().to_string());
            // TODO: Check for notes and update style
            button.connect_clicked(|_| {
                // TODO: Load notes
                println!("test");
            });
            if current_date.date_naive() == self.date.date_naive() {
                today_row = current_week;
                today_col = calendar::days_from_start(&current_date, self.start_sun).try_into().unwrap();
                button.add_css_class("today");
            }

            days_grid.attach(
                &button,
                calendar::days_from_start(&current_date, self.start_sun).try_into().unwrap(),
                current_week,
                1,
                1
            );

            if current_date.weekday() == calendar::last_day_of_week(self.start_sun) {
                current_week += 1;
            }
            current_date += Duration::days(1);
        }

        // next month fill
        while calendar::days_from_start(&current_date, self.start_sun) > 0 {
            // TODO: Add button funcitons
            let button = Button::with_label(&current_date.day().to_string());
            // TODO: Check for notes and update style
            button.set_sensitive(false);
            button.add_css_class("disabled-day");
            days_grid.attach(
                &button,
                calendar::days_from_start(&current_date, self.start_sun).try_into().unwrap(),
                current_week,
                1,
                1
            );
            current_date += Duration::days(1);
        }


        self.window.set_child(Some(&page_grid));

        // focus on today
        days_grid.child_at(today_col, today_row).unwrap().grab_focus();
    }
}

