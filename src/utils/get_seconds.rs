use chrono::prelude::*;
use chrono::{DateTime,Utc};

pub fn get_seconds(date_time: DateTime<Utc>) -> u64 {
    let date_time_year = date_time.year() as u64;
    let date_time_month = date_time.month() as u64;
    let date_time_day = date_time.day() as u64;
    let date_time_hour = date_time.hour() as u64;
    let date_time_minute = date_time.minute() as u64;
    let date_time_second = date_time.second() as u64;
    date_time_year * 365*24*3600 + date_time_month*30*24*3600 + date_time_day*24*3600 + date_time_hour*3600 + date_time_minute*60 + date_time_second
}
