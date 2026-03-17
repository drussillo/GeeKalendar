use chrono::prelude::*;

pub fn get_date_test() {
    let utc: DateTime<Local> = Local::now();
    println!("{}", utc.day());
}

