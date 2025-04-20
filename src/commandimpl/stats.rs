use colored::Colorize;

use crate::utility::{conjucture::deserialize, finddir::find_todo_dir};


pub fn stats() -> Result<(), anyhow::Error> {
    let data_path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found. Run `todo init` to initialize"))?
        .join("todos.bin");
    let task_list = deserialize(&data_path)?;
    let mut p_low = 0;
    let mut p_medium = 0;
    let mut p_high = 0;
    let mut completed = 0;
    let mut not_completed = 0;
    for task in &task_list.tasks {
        match task.priority.as_str() {
            "High" => p_high += 1,
            "Medium" => p_medium += 1,
            _ => p_low += 1,
        }
        if task.completed {
            completed += 1;
        } else {
            not_completed += 1;
        }
    }
    println!("{}", "Stats".bold());
    println!("Total: {}", task_list.tasks.len().to_string().cyan());
    println!("Completed: {} | Not Completed: {}", completed.to_string().green(), not_completed.to_string().red());
    println!("Priority: High: {} | Medium: {} | Low: {}", p_high.to_string().red(), p_medium.to_string().blue(), p_low.to_string().cyan());
    Ok(())
}