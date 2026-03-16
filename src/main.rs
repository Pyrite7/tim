use std::{env, fs, path::PathBuf, str::FromStr};

use anyhow::{Result, anyhow};
use tim::{schedule_tasks, util};
use tim::{Task, cli::{Args, Subcommand}, print_info_table};






fn main() -> Result<()> {
    let data_dir = PathBuf::from_str(&env::var("TIM_DATA_DIR")?)?;
    let args: Args = argh::from_env();

    match args.subcommand {
        Subcommand::Add(add) => {
            let task = add.task(util::now())?;
            task.save(&data_dir)?;
            println!("added task \"{}\"", task.name)
        }

        Subcommand::Ls(ls) => {
            print_info_table(&data_dir, ls.only_undone, ls.all_columns)?;
        }

        Subcommand::Start(start) => {
            let file_contents = &fs::read_to_string(data_dir.join(start.name.clone() + ".json"))
                .map_err(|_| anyhow!("task \"{}\" not found", start.name))?;
            let mut task: Task = serde_json::from_str(file_contents)?;
            task.started_at = Some(util::now());
            task.save(&data_dir)?;
            println!("marked task \"{}\" as started", task.name)
        }

        Subcommand::Done(done) => {
            let file_contents = &fs::read_to_string(data_dir.join(done.name.clone() + ".json"))
                .map_err(|_| anyhow!("task \"{}\" not found", done.name))?;
            let mut task: Task = serde_json::from_str(file_contents)?;
            task.finished_at = Some(util::now());
            task.save(&data_dir)?;
            println!("marked task \"{}\" as done", task.name);
        }

        Subcommand::Rm(rm) => {
            // let mut candidates = HashMap::new();
            // for entry in fs::read_dir(data_dir)? {
            //     let path = entry?.path();
            //     let contents = fs::read_to_string(&path)?;
            //     let task: Task = serde_json::from_str(&contents)?;
            //     if task.name.contains(&rm.name) {
            //         candidates.insert(task, path);
            //     }
            // }
            // if candidates.len() == 0 {
            //     bail!("no tasks matching \"{}\" found", rm.name);
            // } else if candidates.len() == 1 {
            //     fs::remove_file(candidates.values().next().unwrap())?;
            //     println!("removed task \"{}\"", candidates.keys().next().unwrap().name);
            // } else {
            //     todo!()
            // }
            fs::remove_file(data_dir.join(rm.name.clone() + ".json"))
                .map_err(|_| anyhow!("task \"{}\" not found", rm.name))?;
            println!("removed task \"{}\"", rm.name)
        }

        Subcommand::Sch(_) => {
            schedule_tasks(&data_dir)?;
            println!("scheduled tasks")
        }
    }

    Ok(())
}
