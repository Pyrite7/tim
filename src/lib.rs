pub mod cli;
pub mod util;
pub mod expr;


use std::{fs, path::Path};

use anyhow::{Result, ensure};
use chrono::{Duration, NaiveTime, TimeDelta};
use serde::{Deserialize, Serialize};
use crate::util::DateTime;




#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, Hash)]
pub struct Task {
    pub name: String,
    pub deadline: Option<DateTime>,
    pub specified_duration: Option<Duration>,
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

    // Save the task to the given data directory. (will overwrite any existing task with the same name!)
    pub fn save(&self, data_dir: &Path) -> Result<()> {
        Ok(fs::write(data_dir.join(self.file_name()), serde_json::to_string_pretty(&self)?)?)
    }

    pub fn estimated_duration(&self) -> TimeDelta {
        self.specified_duration.unwrap_or(TimeDelta::hours(1))
    }

    pub fn scheduled_end(&self) -> Option<DateTime> {
        Some(self.scheduled_start? + self.estimated_duration())
    }
}


/// Loads all tasks from the given data directory. Fails if even one of the tasks fails to load.
pub fn load_all_tasks(data_dir: &Path) -> Result<Vec<Task>> {
    let mut tasks: Vec<Task> = Vec::new();
    for entry in fs::read_dir(data_dir)? {
        let path = entry?.path();
        tasks.push(serde_json::from_str(&fs::read_to_string(path)?)?);
    }
    Ok(tasks)
}


/// Attempts to load all tasks from the given data directory.
pub fn load_tasks(data_dir: &Path) -> Result<Vec<Result<Task>>> {
    let mut result = Vec::new();
    for entry in fs::read_dir(data_dir)? {
        let catch_errors = || {
            let path = entry?.path();
            let task: Task = serde_json::from_str(&fs::read_to_string(path)?)?;
            Ok(task)
        };
        result.push(catch_errors());
    }

    Ok(result)
}


/// Automatically schedules all tasks.
pub fn schedule_tasks(data_dir: &Path) -> Result<()> {
    // Load all undone tasks
    let mut tasks: Vec<_> = load_all_tasks(data_dir)?
        .into_iter()
        .filter(|task| task.finished_at.is_none())
        .collect();

    // Sort by closest deadline
    tasks.sort_unstable_by_key(|task| task.deadline.unwrap_or(DateTime::MAX_UTC));

    // Starting time (NOTE: ensures that at most 1 task in progress at any given time)
    let tasks_in_progress: Vec<_> = tasks.iter()
        .filter(|task| task.started_at.is_some() && task.finished_at.is_none())
        .collect();
    ensure!(tasks_in_progress.len() <= 1, "at most 1 task can be in progress at a time");
    let starting_time = if let Some(task) = tasks_in_progress.first() {
        task.started_at.unwrap() + task.estimated_duration() 
    } else {
        util::now()
    };

    // Remove in-progress task from scheduling
    tasks.retain(|task| task.started_at.is_none());

    // Schedule
    let mut time_cursor = starting_time;
    for task in tasks.iter_mut() {
        // Check if task fits within working hours
        let working_hours_start = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
        let working_hours_end = NaiveTime::from_hms_opt(18, 0, 0).unwrap();
        if time_cursor.time() < working_hours_start {
            time_cursor = time_cursor
                .date_naive()
                .and_time(working_hours_start)
                .and_utc();
        } else if time_cursor.time() + task.estimated_duration() > working_hours_end {
            time_cursor = time_cursor
                .date_naive()
                .succ_opt().expect("reached end of time")
                .and_time(working_hours_start)
                .and_utc();
        }

        // Schedule task and increment time cursor
        task.scheduled_start = Some(time_cursor);
        time_cursor += task.estimated_duration();

        // Save task
        task.save(&data_dir)?;
    }
    
    Ok(())
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
            task.specified_duration.map(|t| t.to_string()).unwrap_or("-".into()),
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