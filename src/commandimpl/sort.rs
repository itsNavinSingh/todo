use chrono::NaiveDateTime;
use colored::Colorize;

use crate::{
    arguments::{SortBy, SortCommand},
    utility::{conjucture::deserialize, finddir::find_todo_dir},
};

fn priority_task(p: &str) -> u8 {
    match p {
        "High" => 0,
        "Medium" => 1,
        _ => 2,
    }
}

pub fn sort(cmd: &SortCommand) -> Result<(), anyhow::Error> {
    let data_path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found, Run `todo init` to initialize"))?
        .join("todos.bin");
    let mut task_list = deserialize(&data_path)?;
    match cmd.by {
        // by id
        SortBy::Id => task_list.tasks.sort_by(|a, b| a.id.cmp(&b.id)),
        // by title
        SortBy::Title => task_list.tasks.sort_by_key(|item| item.title.clone()),
        // by complete
        SortBy::Complete => task_list.tasks.sort_by_key(|item| !item.completed),
        // by priority
        SortBy::Priority => task_list
            .tasks
            .sort_by_key(|task| priority_task(task.priority.as_str())),
        // by creation
        SortBy::Creation => task_list.tasks.sort_by(|a, b| {
            let a_parsed =
                NaiveDateTime::parse_from_str(a.created_at.as_str(), "%Y-%m-%d %H:%M:%S");
            let b_parsed =
                NaiveDateTime::parse_from_str(b.created_at.as_str(), "%Y-%m-%d %H:%M:%S");
            match (a_parsed, b_parsed) {
                (Ok(a_dt), Ok(b_dt)) => a_dt.cmp(&b_dt),
                _ => std::cmp::Ordering::Less,
            }
        }),
        // by Due
        SortBy::Due => task_list.tasks.sort_by(|a, b| {
            let a_parsed =
                NaiveDateTime::parse_from_str(a.created_at.as_str(), "%Y-%m-%d %H:%M:%S");
            let b_parsed =
                NaiveDateTime::parse_from_str(b.created_at.as_str(), "%Y-%m-%d %H:%M:%S");
            match (a_parsed, b_parsed) {
                (Ok(a_dt), Ok(b_dt)) => a_dt.cmp(&b_dt),
                _ => std::cmp::Ordering::Less,
            }
        }),
    }
    if task_list.tasks.is_empty() {
        println!("{}", "No task found.".yellow());
        return Ok(());
    }
    println!("{}", "Tasks".bold());
    for task in task_list.tasks {
        let status = if task.completed {
            "Done".green()
        } else {
            "Pending".red()
        };
        println!(
            "ID: {} | Title: {} | Status: {} | Priority: {} | Due: {} | CreatedAt: {}",
            task.id.to_string().cyan(),
            task.title,
            status,
            task.priority,
            task.due,
            task.created_at
        );
    }
    Ok(())
}

