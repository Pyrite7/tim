use std::{collections::HashMap, env, fs, path::PathBuf, str::FromStr};

use anyhow::{Result, anyhow, bail};
use chrono::Utc;
use tim::{Task, cli::{Args, Subcommand}, print_info_table};






fn main() -> Result<()> {
    let data_dir = PathBuf::from_str(&env::var("TIM_DATA_DIR")?)?;
    let args: Args = argh::from_env();

    match args.subcommand {
        Subcommand::Add(add) => {
            let task = add.task(Utc::now())?;
            fs::write(data_dir.join(task.file_name()), serde_json::to_string(&task)?)?;
            println!("added task \"{}\"", task.name)
        }

        Subcommand::Ls(ls) => {
            print_info_table(&data_dir, ls.only_undone, ls.all_columns)?;
        }

        Subcommand::Start(start) => {
            let file_contents = &fs::read_to_string(data_dir.join(start.name.clone() + ".json"))
                .map_err(|_| anyhow!("task \"{}\" not found", start.name))?;
            let mut task: Task = serde_json::from_str(file_contents)?;
            task.started_at = Some(Utc::now());
            fs::write(data_dir.join(task.file_name()), serde_json::to_string(&task)?)?;
            println!("marked task \"{}\" as started", task.name)
        }

        Subcommand::Done(done) => {
            let file_contents = &fs::read_to_string(data_dir.join(done.name.clone() + ".json"))
                .map_err(|_| anyhow!("task \"{}\" not found", done.name))?;
            let mut task: Task = serde_json::from_str(file_contents)?;
            task.finished_at = Some(Utc::now());
            fs::write(data_dir.join(task.file_name()), serde_json::to_string(&task)?)?;
            println!("marked task \"{}\" as done", task.name);
        }

        Subcommand::Rm(rm) => {
            let mut candidates = HashMap::new();
            for entry in fs::read_dir(data_dir)? {
                let path = entry?.path();
                let contents = fs::read_to_string(&path)?;
                let task: Task = serde_json::from_str(&contents)?;
                if task.name.contains(&rm.name) {
                    candidates.insert(task, path);
                }
            }
            if candidates.len() == 0 {
                bail!("no tasks matching \"{}\" found", rm.name);
            } else if candidates.len() == 1 {
                fs::remove_file(candidates.values().next().unwrap())?;
                println!("removed task \"{}\"", candidates.keys().next().unwrap().name);
            } else {
                todo!()
            }
        }

        _ => ()
    }

    Ok(())
}
