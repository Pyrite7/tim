pub mod cli;
pub mod util;
pub mod expr;


use chrono::Duration;
use serde::{Deserialize, Serialize};
use crate::util::DateTime;




#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub deadline: Option<DateTime>,
    pub estimated_duration: Option<Duration>,
    pub scheduled_start: Option<DateTime>,
    pub started_at: Option<DateTime>,
    pub finished_at: Option<DateTime>,
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
