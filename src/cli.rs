use argh::FromArgs;




#[derive(Debug, FromArgs, PartialEq, Eq)]
/// Simple time management tool with automatic task scheduling.
pub struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,
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
    name: String,

    /// deadline for task
    #[argh(option, short = 'd')]
    deadline: Option<String>,
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
    name: String,
}

#[derive(Debug, FromArgs, PartialEq, Eq)]
/// Mark a task as started
#[argh(subcommand, name = "start")]
pub struct Start {
    #[argh(positional)]
    name: String,
}
