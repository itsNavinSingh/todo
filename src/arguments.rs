use chrono::NaiveDateTime;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    List,
    Add(AddCommand),
    Delete(DeleteCommand),
    Clear,
    Edit(EditCommand),
    Search(SearchCommand),
    Sort(SortCommand),
    Stats,
    Export(ExportCommand),
}

// Add SubCommand
#[derive(PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Copy, Clone)]
enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Args, Debug)]
struct AddCommand {
    #[arg(short, long)]
    title: String,

    #[arg(value_enum, default_value_t = Priority::Low)]
    priority: Priority,

    #[arg(value_parser = parse_datetime)]
    due: NaiveDateTime,

    #[arg(long, short)]
    complete: bool,
}

fn parse_datetime(s: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(s, "%d-%m-%Y %H:%M:%S")
        .map_err(|e| format!("Invalid datetime: {}", e))
}

// Delete Subcommand
#[derive(Args, Debug)]
struct DeleteCommand {
    id: u32,
}

// Edit Subcommand

#[derive(Args, Debug)]
struct EditCommand {
    id: u32,

    #[arg(long, short)]
    title: Option<String>,

    #[arg(value_enum)]
    priority: Option<Priority>,

    #[arg(value_parser = parse_datetime)]
    due: Option<NaiveDateTime>,

    #[arg(long, short)]
    complete: Option<bool>,
}

// Search
#[derive(Args, Debug)]
struct SearchCommand {
    word: String,
}

// sort
#[derive(PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Copy, Clone)]
enum SortBy {
    Id,
    Title,
    Priority,
    CreatedAt,
    Due,
}
#[derive(Args, Debug)]
struct SortCommand {
    #[arg(value_enum, short, long)]
    by: SortBy,
}

// export
#[derive(PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Copy, Clone)]
enum Format {
    CSV,
    JSON,
}
#[derive(Args, Debug)]
struct ExportCommand {
    #[arg(short, long, default_value = "output")]
    output: String,
    #[arg(value_enum, short, long, default_value_t = Format::CSV)]
    format: Format,
}
