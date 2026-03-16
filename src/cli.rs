use anyhow::Result;
use argh::FromArgs;

use crate::{Task, expr::{TimeDeltaExpr, TimeExpr}, util::DateTime};




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
    Rm(Rm),
}

#[derive(Debug, FromArgs, PartialEq, Eq)]
/// List tasks
#[argh(subcommand, name = "ls")]
pub struct Ls {
    /// show all columns
    #[argh(switch, short = 'a')]
    pub all_columns: bool,

    /// only show undone tasks
    #[argh(switch, short = 'u')]
    pub only_undone: bool,
}

#[derive(Debug, FromArgs, PartialEq, Eq)]
/// Add a task
#[argh(subcommand, name = "add")]
pub struct Add {
    #[argh(positional)]
    pub name: String,

    /// deadline for task
    #[argh(option, short = 'd')]
    deadline: Option<TimeExpr>,

    /// estimated time taken
    #[argh(option, short = 't')]
    takes: Option<TimeDeltaExpr>,
}

impl Add {
    /// Construct the task to be added by this subcommand
    pub fn task(&self, now: DateTime) -> Result<Task> {
        let mut result = Task::new(&self.name);
        if let Some(deadline) = &self.deadline {
            result.deadline = Some(deadline.eval(now)?)
        }
        if let Some(takes) = &self.takes {
            result.specified_duration = Some(takes.eval())
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

#[derive(Debug, FromArgs, PartialEq, Eq)]
/// Remove a task
#[argh(subcommand, name = "rm")]
pub struct Rm {
    #[argh(positional)]
    pub name: String,
}