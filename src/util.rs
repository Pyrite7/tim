use chrono::{Local, NaiveDate, NaiveTime, Utc};
use anyhow::{Result, anyhow};



pub type DateTime = chrono::DateTime<Utc>;

/// Returns the current time
pub fn now() -> DateTime {
    Utc::now()
}

pub fn combine_dt(date: NaiveDate, time: NaiveTime) -> DateTime {
    date
        .and_time(time)
        .and_utc()
}


pub type IODateTime = chrono::DateTime<Local>;

pub fn to_io(utc: DateTime) -> IODateTime {
    utc.with_timezone(&Local)
}

pub fn to_utc(io: IODateTime) -> DateTime {
    io.to_utc()
}

pub fn combine_io_dt(date: NaiveDate, time: NaiveTime) -> Result<IODateTime> {
    Ok(date
        .and_time(time)
        .and_local_timezone(Local)
        .earliest()
        .ok_or(anyhow!("time {time} does not exist on {date}"))?)
}

pub fn combine_io_dt_to_utc(date: NaiveDate, time: NaiveTime) -> Result<DateTime> {
    combine_io_dt(date, time).map(to_utc)
}
