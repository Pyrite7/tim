pub mod cli;
pub mod util;
pub mod expr;


use std::{fs, path::Path};

use anyhow::Result;
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


/// Print an info listing for tasks.
pub fn print_info_table(data_dir: &Path, only_undone_tasks: bool, show_all_columns: bool) -> Result<Vec<anyhow::Error>> {
    let columns = if show_all_columns { vec![
        "Name",
        "Deadline",
        "Estimated duration",
        "Scheduled start",
        "Started at",
        "Finished at",
    ] } else { vec![
        "Name",    
        "Deadline",    
        "Estimated duration",    
        "Scheduled start",    
    ] };

    let columns_of_task = |task: &Task| {
        let mut cols = vec![
            task.name.clone(),
            task.deadline.map(|t| t.to_string()).unwrap_or("-".into()),
            task.estimated_duration.map(|t| t.to_string()).unwrap_or("-".into()),
            task.scheduled_start.map(|t| t.to_string()).unwrap_or("-".into()),
            task.started_at.map(|t| t.to_string()).unwrap_or("-".into()),
            task.finished_at.map(|t| t.to_string()).unwrap_or("-".into()),
        ];
        if !show_all_columns {
            // started_at
            cols.remove(4);
            // finished_at (note that index has changed to 4 because 'started_at' was removed)
            cols.remove(4);
        }
        return cols
    };

    let mut column_widths: Vec<usize> = columns.iter().map(|col_name| col_name.chars().count()).collect();
    
    let mut tasks = Vec::new();
    let mut file_errors = Vec::new();
    for file in fs::read_dir(data_dir)? {
        let catch_errors = || {
            let file = file?;
            let task: Task = serde_json::from_str(&fs::read_to_string(file.path())?)?;

            return Ok::<_, anyhow::Error>(task);
        };

        match catch_errors() {
            Ok(task) => {
                if only_undone_tasks && task.finished_at.is_some() {
                    continue;
                }

                let cols_of_task = columns_of_task(&task);
                for (col, w) in cols_of_task.iter().zip(column_widths.iter_mut()) {
                    let col_w = col.chars().count();
                    if col_w > *w {
                        *w = col_w;
                    }
                }
                tasks.push(task)
            },
            Err(err) => file_errors.push(err),
        }
    }

    let print_row = |cols: &[&str]| {
        for (n, col) in cols.iter().enumerate() {
            let width = column_widths[n];
            if *col == "-" {
                // Center the dash in empty columns
                print!("| {:^width$} ", col);
            } else {
                print!("| {:<width$} ", col);
            }
        }

        println!("|");
    };

    print_row(&columns);
    let separator_cols: Vec<_> = column_widths.iter().map(|w| "-".repeat(*w)).collect();
    print_row(&separator_cols.iter().map(|s| &s as &str).collect::<Vec<_>>());
    
    tasks.sort_by_key(|task| task.deadline);
    for task in tasks {
        let cols = columns_of_task(&task);
        let cols: Vec<_> = cols.iter().map(|s| &s as &str).collect();
        print_row(&cols);
    }

    Ok(file_errors)
}