use std::{env, fs, path::PathBuf, str::FromStr};

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    Task,
    expr::{TimeDeltaExpr, TimeExpr},
    print_info_table, schedule_tasks, util,
};

#[derive(Debug, Parser)]
#[command(version, about, name = "tim")]
pub struct Cli {
    #[arg(long, short)]
    pub data_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub sub_cmd: SubCmd,
}

#[derive(Debug, Subcommand)]
pub enum SubCmd {
    Ls {
        #[arg(long, short)]
        all: bool,
    },
    Add {
        name: String,

        #[arg(long, short)]
        deadline: Option<TimeExpr>,

        #[arg(long, short)]
        takes: Option<TimeDeltaExpr>,
    },
    #[command(visible_alias = "sch")]
    Schedule,
    Done {
        name: String,
    },
    Start {
        name: String,
    },
    #[command(visible_alias = "rm")]
    Remove {
        names: Vec<String>,
    },
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        let data_dir = self.data_dir.unwrap_or(PathBuf::from_str(&env::var("TIM_DATA_DIR")?)?);

        match self.sub_cmd {
            SubCmd::Ls { all } => {
                print_info_table(&data_dir, !all, all)?;
            }
            SubCmd::Add {
                name,
                deadline,
                takes,
            } => {
                let mut task = Task::new(&name);
                if let Some(dl) = deadline {
                    task.deadline = Some(dl.eval(util::now())?);
                }
                if let Some(takes) = takes {
                    task.specified_duration = Some(takes.eval());
                }
                task.save(&data_dir)?;
            }
            SubCmd::Schedule => {
                schedule_tasks(&data_dir)?;
            }
            SubCmd::Start { name } => {
                let mut task: Task =
                    serde_json::from_str(&fs::read_to_string(data_dir.join(name + ".json"))?)?;
                task.started_at = Some(util::now());
                task.save(&data_dir)?;
            }
            SubCmd::Done { name } => {
                let mut task: Task =
                    serde_json::from_str(&fs::read_to_string(data_dir.join(name + ".json"))?)?;
                task.finished_at = Some(util::now());
                task.save(&data_dir)?;
            }
            SubCmd::Remove { names } => {
                for name in names {
                    fs::remove_file(data_dir.join(name + ".json"))?;
                }
            }
        }

        Ok(())
    }
}
