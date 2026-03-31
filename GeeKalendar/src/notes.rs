
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use directories::BaseDirs;


// enum Color {
//     RED,
//     GREEN,
//     BLUE,
// }

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub date: String,
    pub title: String,
    pub message: String
}

impl Note {
    pub fn new(date: DateTime<Local>, title: &str, message: &str) -> Self {
        Note {
            date: date.to_rfc3339(),
            title: title.to_owned(),
            message: message.to_owned()
        }
    }
}


pub fn read_notes(date: &DateTime<Local>) -> Option<Vec<Note>> {
    let filename = date.date_naive().to_string();
    // let path = format!("resources/notes/{}.json", filename);

    let mut path = BaseDirs::new()
        .unwrap()
        .data_dir()
        .to_path_buf();

    path.push("GeeKalendar/notes/");
    if !path.is_dir() {
        if let Err(e) = std::fs::create_dir_all(&path) {
            println!("Error creating path {}: {}", path.display(), e);
            return None;
        }
    }

    path.push(format!("{}.json", filename));
    let filestr = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str::<Vec<Note>>(&filestr).ok()
}


pub fn write_notes(notes: &Vec<Note>) {
    let date = DateTime::parse_from_rfc3339(&notes[0].date).unwrap();

    let mut path = BaseDirs::new()
        .unwrap()
        .data_dir()
        .to_path_buf();

    path.push("GeeKalendar/notes/");
    if !path.is_dir() {
        if let Err(e) = std::fs::create_dir_all(&path) {
            println!("Error creating path {}: {}", path.display(), e);
            return
        }
    }

    path.push(format!("{}.json", date.date_naive()));
    match serde_json::to_string_pretty(notes) {
        Err(e) => println!("Error: could not serialize vector for {}: {}", date.date_naive(), e),
        Ok(j) => {
            if let Err(e) = std::fs::write(&path, j) {
                println!("Error: could not write to file {}: {}", path.display(), e);
            }
        }
    }
}


