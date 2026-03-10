use anyhow::Result;
use argh::FromArgs;

use crate::{expr::DateTimeExpr, Task, util::DateTime};




#[derive(Debug, FromArgs, PartialEq, Eq)]
/// Simple time management tool with automatic task scheduling.
pub struct Args {
    #[argh(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, FromArgs, PartialEq, Eq)]
#[argh(subcommand)]
pub enum Subcommand {
    Ls(Ls),
    Add(Add),
    Sch(Sch),
    Done(Done),
    Start(Start),
}

#[derive(Debug, FromArgs, PartialEq, Eq)]
/// List tasks
#[argh(subcommand, name = "ls")]
pub struct Ls {}

#[derive(Debug, FromArgs, PartialEq, Eq)]
/// Add a task
#[argh(subcommand, name = "add")]
pub struct Add {
    #[argh(positional)]
    pub name: String,

    /// deadline for task
    #[argh(option, short = 'd')]
    deadline: Option<DateTimeExpr>,
}

impl Add {
    /// Construct the task to be added by this subcommand
    pub fn task(&self, now: DateTime) -> Result<Task> {
        let mut result = Task::new(&self.name);
        if let Some(deadline) = &self.deadline {
            result.deadline = Some(deadline.eval(now)?)
        }
        Ok(result)
    }
}

#[derive(Debug, FromArgs, PartialEq, Eq)]
/// Schedule tasks
#[argh(subcommand, name = "sch")]
pub struct Sch {}

#[derive(Debug, FromArgs, PartialEq, Eq)]
/// Mark a task as done
#[argh(subcommand, name = "done")]
pub struct Done {
    #[argh(positional)]
    pub name: String,
}

#[derive(Debug, FromArgs, PartialEq, Eq)]
/// Mark a task as started
#[argh(subcommand, name = "start")]
pub struct Start {
    #[argh(positional)]
    pub name: String,
}
