use std::{env, fs, path::PathBuf, str::FromStr};

use anyhow::{Result, anyhow};
use chrono::Utc;
use tim::{cli::{Args, Subcommand}, Task};






fn main() -> Result<()> {
    let data_dir = PathBuf::from_str(&env::var("TIM_DATA_DIR")?)?;
    let args: Args = argh::from_env();

    match args.subcommand {
        Subcommand::Add(add) => {
            let task = add.task(Utc::now())?;
            fs::write(data_dir.join(task.file_name()), serde_json::to_string(&task)?)?;
        }
        Subcommand::Ls(_) => {
            for file in fs::read_dir(data_dir)? {
                let file = file?;
                if file.file_name().to_string_lossy().ends_with(".json") {
                    let task: Task = serde_json::from_str(&fs::read_to_string(file.path())?)?;
                    println!("{}", task.name)
                }
            }
        }
        Subcommand::Start(start) => {
            let file_contents = &fs::read_to_string(data_dir.join(start.name.clone() + ".json"))
                .map_err(|_| anyhow!("task \"{}\" not found", start.name))?;
            let mut task: Task = serde_json::from_str(file_contents)?;
            task.started_at = Some(Utc::now());
            fs::write(data_dir.join(task.file_name()), serde_json::to_string(&task)?)?;
        }
        Subcommand::Done(done) => {
            let file_contents = &fs::read_to_string(data_dir.join(done.name.clone() + ".json"))
                .map_err(|_| anyhow!("task \"{}\" not found", done.name))?;
            let mut task: Task = serde_json::from_str(file_contents)?;
            task.finished_at = Some(Utc::now());
            fs::write(data_dir.join(task.file_name()), serde_json::to_string(&task)?)?;
        }
        _ => ()
    }

    Ok(())
}
