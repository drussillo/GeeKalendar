use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{CssProvider, Button, Grid, Label, ScrolledWindow, Box, Overlay, Entry};
use chrono::prelude::*;
use chrono::{Duration};
use regex::Regex;

use crate::calendar;
use crate::notes;

pub fn set_style() {
    let provider = CssProvider::new();
    provider.load_from_path("/home/dave/code/GeeKalendar/GeeKalendar/resources/style.css");  // TODO resource path

    let display = gtk::gdk::Display::default().expect("No display");
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

impl calendar::Page {

    pub fn make_page(&self) {
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

        // notes scrolled window
        let notes_window = ScrolledWindow::builder()
            .sensitive(false)
            .build();
        page_grid.attach(&notes_window, 7, 0, 4, 7);
        // TODO: add style?


        // calendar days 
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

            if let Some(_) = notes::read_notes(&current_date) {
                button.add_css_class("day-with-note");
            }

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
            let button = Button::with_label(&current_date.day().to_string());

            // attach date to button object
            unsafe {
                button.set_data("date", current_date);
            }

            if let Some(_) = notes::read_notes(&current_date) {
                button.add_css_class("day-with-note");
            }

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

            if let Some(_) = notes::read_notes(&current_date) {
                button.add_css_class("day-with-note");
            }

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

        // overlay
        let note_overlay = Overlay::builder()
            .child(&page_grid)
            .build();

        // self.window.set_child(Some(&page_grid));
        self.window.set_child(Some(&note_overlay));

        // focus on today and display notes
        days_grid.child_at(today_col, today_row).unwrap().grab_focus();
        self.list_current_notes();
    }

    pub fn list_current_notes(&self) {
        // get date
        let date: &DateTime<Local>;
        unsafe {
            date = gtk4::prelude::GtkWindowExt::focus(&self.window)
                .unwrap()
                .data::<DateTime<Local>>("date")
                .unwrap()
                .as_ref();
        }

        let notes_box = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .spacing(5)
            .build();

        // get notes vector
        if let Some(current_notes) = notes::read_notes(date) {
            for note in current_notes {
                notes_box.append(&note.get_box());
            }
        } else {
            notes_box.set_homogeneous(true);
            notes_box.append(
                &Label::builder()
                    .label(&format!("no notes available\nfor {}", date.date_naive()))
                    .justify(gtk::Justification::Center)
                    .build()
            );
        }

        self.window
            .child()
            .and_downcast::<Overlay>()
            .unwrap()
            .child()
            .and_downcast::<Grid>()
            .unwrap()
            .child_at(7, 0)
            .and_downcast::<ScrolledWindow>()
            .unwrap()
            .set_child(Some(&notes_box));
    }


    pub fn add_note(self) {
        // extract date
        let button = &gtk::prelude::GtkWindowExt::focus(&self.window).unwrap();
        let date: &DateTime<Local>;
        unsafe {
            date = button
                .data::<DateTime<Local>>("date")
                .unwrap()
                .as_ref();
        }

        let overlay_ref = &self.window
            .child()
            .and_downcast::<Overlay>()
            .unwrap();

        // disable background
        overlay_ref
            .child()
            .and_downcast::<Grid>()
            .unwrap()
            .set_sensitive(false);

        // create input overlay
        let title_input = Entry::builder()
            .placeholder_text("title")
            .margin_top(10)
            .margin_bottom(10)
            .margin_end(10)
            .margin_start(10)
            .build();

        let message_input = Entry::builder()
            .placeholder_text("message")
            .margin_top(10)
            .margin_bottom(10)
            .margin_end(10)
            .margin_start(10)
            .build();

        // todo maybe use textview?
        // let message_input = TextView::builder()
        //     .margin_top(10)
        //     .margin_bottom(10)
        //     .margin_end(10)
        //     .margin_start(10)
        //     .height_request(100)
        //     .accepts_tab(false)
        //     .build();

        let input_box = Box::builder()
            .css_classes(vec!("note-overlay"))
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .orientation(gtk::Orientation::Vertical)
            .build();
        input_box.append(&Label::builder()
            .use_markup(true)
            .label(format!("<b>NEW NOTE FOR {}</b>", date.date_naive()))
            .margin_top(10)
            .margin_bottom(10)
            .margin_end(10)
            .margin_start(10)
            .build());
        input_box.append(&title_input);
        input_box.append(&message_input);

        overlay_ref.add_overlay(&input_box);
        title_input.grab_focus();

        // set signals
        let overlay_clone = overlay_ref.clone();
        let input_box_clone = input_box.clone();
        let title_input_clone1 = title_input.clone();
        let title_input_clone2 = title_input.clone();
        let message_input_clone = message_input.clone();
        let grid_clone = overlay_ref
            .child()
            .and_downcast::<Grid>()
            .unwrap()
            .clone();
        let self_clone = self.clone();
        // let button_clone = button.clone();

        message_input.connect_activate(move |_| {
            title_input_clone1.emit_activate();
        });

        title_input.connect_activate(move |_| {
            let new_title = title_input_clone2.text();
            let title_re = Regex::new("^.{1,30}$").unwrap();
            let new_message = message_input_clone.text();

            if !title_re.is_match(&new_title) {
                title_input_clone2.set_text("");
                // TODO: add css??
                title_input_clone2.set_placeholder_text(Some("Invalid title"));
                return
            }

            overlay_clone.remove_overlay(&input_box_clone);
            grid_clone.set_sensitive(true);

            // add note
            let new_note = notes::Note::new(date, &new_title, &new_message);
            let mut notes = notes::read_notes(&date).unwrap_or_default();
            notes.push(new_note);
            notes::write_notes(&notes);

            // update UI
            self_clone.make_page();
        });
    }


    pub fn has_overlay(&self) -> bool {
        self.window
            .child()
            .and_downcast::<Overlay>()
            .unwrap()
            .observe_children()
            .n_items() > 1
    }

    // fn focus_on_date(&self, date: &DateTime<Local>) {
    //     let col = calendar::days_from_start(date, self.start_sun);
    //     let row = 
    // }
}


impl notes::Note {
    pub fn get_box(&self) -> Box {
        let note_box = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(vec!("todo-note-box"))
            .build();

        // title
        note_box.append(&Label::builder()
            // .css_classes(vec!(""))
            .margin_top(10)
            .margin_bottom(5)
            .justify(gtk::Justification::Center)
            .use_markup(true)
            .label(format!("\
                <b>{}</b>\
                ", self.title))
            .build());

        // message
        note_box.append(&Label::builder()
            // .justify(gtk::Justification::Left)
            .halign(gtk::Align::Start)
            .margin_bottom(10)
            .margin_start(10)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .use_markup(true)
            .label(format!("\
                {}\
                ", self.message))
            .build());

        note_box
    }
}


