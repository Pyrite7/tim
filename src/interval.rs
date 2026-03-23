use chrono::Duration;

use crate::util::DateTime;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub begin: DateTime,
    pub end: DateTime,
}

impl Interval {
    pub fn new(begin: DateTime, end: DateTime) -> Self {
        Self { begin, end }
    }

    pub fn from_duration(begin: DateTime, duration: Duration) -> Self {
        Self::new(begin, begin + duration)
    }

    /// Returns the overlap between two intervals, if there is any.
    pub fn overlap(&self, other: Interval) -> Option<Interval> {
        if self.begin <= other.begin && other.begin < self.end {
            Some(Self::new(other.begin, self.end))
        } else if other.begin <= self.begin && self.begin < other.end {
            Some(Self::new(self.begin, other.end))
        } else {
            None
        }
    }

    /// Returns an iterator of intervals in `others` that overlap with `self`. Each item is a pair of the overlapping region and the overlap itself.
    pub fn overlaps(&self, others: impl IntoIterator<Item = Interval>) -> impl Iterator<Item = (Interval, Interval)> {
        others
            .into_iter()
            .filter_map(|i| self.overlap(i).map(|ol| (i, ol)))
    }
}


