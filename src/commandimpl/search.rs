use colored::Colorize;

use crate::{arguments::SearchCommand, utility::{conjucture::deserialize, finddir::find_todo_dir}};


pub fn search(cmd: &SearchCommand) -> Result<(), anyhow::Error> {
    let data_path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found. Run `todo init` to initialize"))?
        .join("todos.bin");

    let task_list = deserialize(&data_path)?;

    for task in &task_list.tasks {
        if task.title.contains(cmd.word.as_str()) {
            let status = if task.completed {
                "Done".green()
            } else {
                "Pending".red()
            };
            println!(
                "ID: {} | Title: {} | Status: {} | Priority: {} | Due: {} | CreatedAt: {}",
                task.id.to_string().cyan(),
                task.title.bold(),
                status,
                task.priority,
                task.due.red(),
                task.created_at.green()
            );
        }
    }
    Ok(())
}