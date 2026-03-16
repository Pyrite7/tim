use std::str::FromStr;

use anyhow::{Error, Result, anyhow, bail};
use chrono::{NaiveDate, NaiveTime, TimeDelta};

use crate::util::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeExpr {
    Now,

    // "Date expressions" are always datetime expressions with time of day 00:00
    Today,
    Tomorrow,
    Date(NaiveDate),

    Add(Box<TimeExpr>, Vec<TimeDeltaExpr>),
}

impl FromStr for TimeExpr {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Quick keywords
        match s {
            "now" => return Ok(Self::Now),
            "today" => return Ok(Self::Today),
            "tmr" | "tomorrow" => return Ok(Self::Tomorrow),
            _ => (),
        }

        // Literal dates
        if let Ok(date) = NaiveDate::parse_from_str(s, "%d.%m.%Y") {
            return Ok(TimeExpr::Date(date));
        }

        // With deltas
        let mut parts = s.split("+");
        let anchor = parts
            .next()
            .ok_or(anyhow!("invalid time expression: {s}"))?;
        let deltas: Vec<_> = parts
            .map(TimeDeltaExpr::from_str)
            .collect::<Result<Vec<TimeDeltaExpr>, Error>>()?;

        let anchor = TimeExpr::from_str(anchor)?;

        if deltas.is_empty() {
            Ok(anchor)
        } else {
            Ok(TimeExpr::Add(Box::new(anchor), deltas))
        }
    }
}

impl TimeExpr {
    pub fn eval(&self, now: DateTime) -> Result<DateTime> {
        match self {
            Self::Now => Ok(now),
            Self::Today => combine_io_dt_to_utc(now.date_naive(), NaiveTime::default()),
            Self::Tomorrow => combine_io_dt_to_utc(
                now.date_naive()
                    .succ_opt()
                    .ok_or(anyhow!("reached end of time"))?,
                NaiveTime::default(),
            ),
            Self::Date(date) => combine_io_dt_to_utc(*date, NaiveTime::default()),
            Self::Add(t, d) => d.iter().fold(Ok(t.eval(now)?), |t, d| Ok(t? + d.eval())),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TimeDeltaExpr {
    TimeOfDay { h: usize, m: usize },
    // TODO: fix
    // TimeAmount {
    //     w: usize,
    //     d: usize,
    //     h: usize,
    //     m: usize,
    //     s: usize,
    // },
}

impl TimeDeltaExpr {
    pub fn eval(&self) -> TimeDelta {
        match self {
            Self::TimeOfDay { h, m } => TimeDelta::minutes((m + h * 60) as _),
            // TODO: fix
            // Self::TimeAmount { w, d, h, m, s } => {
            //     TimeDelta::weeks(*w as _)
            //     + TimeDelta::days(*d as _)
            //     + TimeDelta::hours(*h as _)
            //     + TimeDelta::minutes(*m as _)
            //     + TimeDelta::seconds(*s as _)
            // }
        }
    }
}

impl FromStr for TimeDeltaExpr {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // static TIME_AMOUNT_REGEX: LazyLock<Regex> = LazyLock::new(
        //     || Regex::new(r"(\d+w)?(\d+d)?(\d+h)?(\d+m(?:in)?)?(\d+s)?").unwrap()
        // );

        if s.contains(":") {
            let mut parts = s.split(":");
            let h = parts.next().unwrap_or_default().parse()?;
            let m = parts.next().unwrap_or_default().parse()?;
            Ok(TimeDeltaExpr::TimeOfDay { h, m })
        } else {
            // TODO: fix
            // let caps = TIME_AMOUNT_REGEX.captures(s).ok_or(anyhow!("invalid time delta: {s}"))?;
            // let parse = |m: Match<'_>, c| {
            //     usize::from_str(m.as_str().strip_suffix(c).unwrap())
            // };
            // Ok(TimeDeltaExpr::TimeAmount {
            //     w: caps.get(1).map(|m| parse(m, "w")).unwrap_or(Ok(0))?,
            //     d: caps.get(1).map(|m| parse(m, "d")).unwrap_or(Ok(0))?,
            //     h: caps.get(1).map(|m| parse(m, "h")).unwrap_or(Ok(0))?,
            //     m: caps.get(1).map(|m| parse(m, "m")).unwrap_or(Ok(0))?,
            //     s: caps.get(1).map(|m| parse(m, "s")).unwrap_or(Ok(0))?
            // })
            bail!("invalid time delta: {s}")
        }
    }
}
