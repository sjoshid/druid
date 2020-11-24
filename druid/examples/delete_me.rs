use chrono::{NaiveDate, DateTime, Local, Datelike};

fn main() {
    let local: DateTime<Local> = Local::now();
    let nd = NaiveDate::from_ymd(local.year(), local.month(), 1);
    println!("{:?}", nd.weekday().num_days_from_sunday());
}