use std::io::Error;
use jiff::civil::{Date};

pub struct DateTime {}

impl DateTime {
    pub fn parse_raw_date(&self, formatted_date: &str) -> Result<Date, Error> {
        todo!("DateTime::parse")
    }

    pub fn get_date_span_in_years(&self, comparable_date_from: Date, comparable_date_to: Date) -> Result<i16, Error> {
        todo!("DateTime::get_date_time_span")
    }
}
