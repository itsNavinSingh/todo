use colored::Colorize;

use crate::utility::{conjucture::deserialize, finddir::find_todo_dir};

pub fn list() -> Result<(), anyhow::Error> {
    let data_path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found. Run `todo init` to initialize"))?
        .join("todos.bin");

    let task_list = deserialize(&data_path)?;

    if task_list.tasks.is_empty() {
        println!("{}", "No task found.".yellow());
        return Ok(());
    }
    println!("{}", "Tasks".bold());
    for task in task_list.tasks {
        let status = if task.completed {
            "✔ Done".green()
        } else {
            "⬜ Pending".red()
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
