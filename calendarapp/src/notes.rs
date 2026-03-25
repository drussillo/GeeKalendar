use serde::{Deserialize, Serialize};
use serde_json::{Value, Result};
use chrono::prelude::*;


enum Color {
    RED,
    GREEN,
    BLUE,
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    date: String,
    title: String,
    message: String
}

impl Note {
    pub fn new(date: DateTime<Local>, title: &str, message: &str) -> Self {
        Note {
            date: date.date_naive().to_string(),
            title: title.to_owned(),
            message: message.to_owned()
        }
    }

    pub fn append_serialize(&self, string: &mut String) -> Result<()> {
        let j = serde_json::to_string_pretty(self)?;
        string.push_str(&j);
        string.push_str("\n");

        Ok(())
    }
}


// pub fn read_notes(date: DateTime<Local>) -> Vec<Note> {
//     let mut result: Vec<Note> = vec!();
//     let current_date = date.date_naive().to_string();
//
//     for entry in std::fs::read_dir("resources/notes/").unwrap() {
//         let filename = entry
//             .unwrap()
//             .file_name()
//             .to_string_lossy()
//             .to_string();
//
//
//     }
//
//     result
// }



