use pulldown_cmark::{html, Options, Parser, OPTION_ENABLE_TABLES};
use timeago;
use chrono::{Datelike, Timelike, NaiveDateTime};

pub mod schema;
pub mod error;
pub mod token;
pub mod jwt;

pub fn markdown_to_html(content: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    let mut capacity = String::with_capacity(content.len() * 3 / 2);
    let result = Parser::new_ext(content, opts);
    html::push_html(&mut capacity, result);
    capacity
}

pub fn time(end: NaiveDateTime, begin: NaiveDateTime) -> String {

    let end_time = ((end.year() as u64)* 365*24*3600) + ((end.month() as u64) *30*24*3600) + ((end.day() as u64) *24*3600)
                 + ((end.hour() as u64) *3600) + ((end.minute() as u64)*60) + (end.second() as u64 );
    let begin_time = ((begin.year() as u64)* 365*24*3600) + ((begin.month() as u64) *30*24*3600) + ((begin.day() as u64)*24*3600)
                   + ((begin.hour() as u64)*3600) + ((begin.minute() as u64)*60) + (begin.second() as u64) ;
    let secs = end_time - begin_time;
    
    let timeago = timeago::Formatter::new();
    let result = ::std::time::Duration::from_secs(secs);
    timeago.convert(result)
}