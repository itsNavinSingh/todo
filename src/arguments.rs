use chrono::NaiveDateTime;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, name = "todo", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize the empty todo
    Init,
    /// List all the task
    List,
    /// Add new task
    Add(AddCommand),
    /// delete the task
    Delete(DeleteCommand),
    /// clear are completed tasks
    Clear,
    /// edit the task
    Edit(EditCommand),
    /// search from tasks
    Search(SearchCommand),
    /// sort the task by parameter
    Sort(SortCommand),
    /// show metadata about the task
    Stats,
    /// export the tasks
    Export(ExportCommand),
    /// Remove the whole todo from this project
    Remove,
}

// Add SubCommand
#[derive(PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Copy, Clone)]
pub enum Priority {
    /// Set priority to High
    High,
    /// Set priority to Medium
    Medium,
    /// Set priority to Low
    Low,
}

#[derive(Args, Debug)]
pub struct AddCommand {
    #[arg(short, long, help = "Set the task title")]
    pub title: String,

    #[arg(value_parser = parse_datetime, help = "Set the due datetime of task. format: DD-MM-YY HH:MM:SS", long, short)]
    pub due: NaiveDateTime,

    #[arg(long, short, help = "Is task completed or not")]
    pub complete: bool,

    #[arg(value_enum, default_value_t = Priority::Low, help = "Set the task priority", long, short)]
    pub priority: Priority,
}

fn parse_datetime(s: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(s, "%d-%m-%Y %H:%M:%S")
        .map_err(|e| format!("Invalid datetime: {}", e))
}

// Delete Subcommand
#[derive(Args, Debug)]
pub struct DeleteCommand {
    /// Task id to delete
    id: u32,
}

// Edit Subcommand

#[derive(Args, Debug)]
pub struct EditCommand {
    /// Task id to edit
    id: u32,

    #[arg(long, short, help = "Set new task title")]
    title: Option<String>,

    #[arg(value_enum, help = "Set new priority")]
    priority: Option<Priority>,

    #[arg(value_parser = parse_datetime, help = "Set new Due DateTime")]
    due: Option<NaiveDateTime>,

    #[arg(long, short, help = "Set new stusts of task")]
    complete: Option<bool>,
}

// Search
#[derive(Args, Debug)]
pub struct SearchCommand {
    /// text to search in task
    word: String,
}

// sort
#[derive(PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Copy, Clone)]
pub enum SortBy {
    /// Sort the task by task id
    Id,
    /// Sort the task by task title
    Title,
    /// Sort the task based on priority of task
    Priority,
    /// Sort the task based on creation time of task
    CreatedAt,
    /// Sort the task based on due date-time of task
    Due,
}
#[derive(Args, Debug)]
pub struct SortCommand {
    #[arg(value_enum, short, long, help = "Sort task by parameter")]
    by: SortBy,
}

// export
#[derive(PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Copy, Clone)]
pub enum Format {
    /// Export task in .csv format
    CSV,
    /// Export tasks in .json format
    JSON,
}
#[derive(Args, Debug)]
pub struct ExportCommand {
    #[arg(short, long, default_value = "output", help = "Sepecify the file name")]
    output: String,
    #[arg(value_enum, short, long, default_value_t = Format::CSV, help = "Specify the file format name")]
    format: Format,
}
