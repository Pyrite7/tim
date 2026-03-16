use crate::util::{DateTime, IODateTime};
use chrono::{Datelike, TimeDelta, Timelike};

pub fn format_datetime(datetime: IODateTime) -> String {
    format!(
        "{h:02}:{m:02}:{s:02} {d}.{mon}.{y}",
        h = datetime.hour(),
        m = datetime.minute(),
        s = datetime.second(),
        d = datetime.day(),
        mon = datetime.month(),
        y = datetime.year(),
    )
}

pub fn format_timedelta(timedelta: TimeDelta) -> String {
    let mut s = String::new();
    let mut remaining = timedelta.clone();

    let weeks = remaining.num_weeks();
    if weeks > 0 {
        s += &format!("{}w", weeks);
        remaining -= TimeDelta::weeks(weeks);
    }

    let days = remaining.num_days();
    if days > 0 && weeks < 2 {
        s += &format!("{}d", days);
        remaining -= TimeDelta::days(days);
    }

    let hours = remaining.num_hours();
    if hours > 0 && days <= 2 && weeks == 0 {
        s += &format!("{}h", hours);
        remaining -= TimeDelta::hours(hours);
    }

    let minutes = remaining.num_minutes();
    if minutes > 0 && days == 0 && weeks == 0 {
        s += &format!("{}m", minutes);
        remaining -= TimeDelta::minutes(minutes);
    }

    let seconds = remaining.num_seconds();
    if minutes < 10 && hours == 0 && days == 0 && weeks == 0 {
        s += &format!("{}s", seconds);
        remaining -= TimeDelta::seconds(seconds);
    }

    s
}

pub fn format_relative_datetime(datetime: IODateTime, now: DateTime) -> String {
    let delta = now.signed_duration_since(datetime);

    if delta < TimeDelta::zero() {
        format!("in {}", format_timedelta(-delta))
    } else {
        format!("{} ago", format_timedelta(delta))
    }
}
