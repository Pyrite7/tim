use std::fmt::Display;

use chrono::Duration;
use serde::{Deserialize, Serialize};
use crate::util::Time;




#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub deadline: Option<Time>,
    pub estimated_duration: Option<Duration>,
    pub scheduled_start: Option<Time>,
    pub started_at: Option<Time>,
    pub finished_at: Option<Time>,
}

impl Task {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), ..Default::default() }
    }

    // Returns the name of the file this task is to be saved in.
    pub fn file_name(&self) -> String {
        self.name.clone() + ".json"
    }
}

