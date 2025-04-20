use std::fs::File;

use colored::Colorize;

use crate::{arguments::{AddCommand, Priority}, tasks::Task, utility::{conjucture::{deserialize, serialize}, finddir::find_todo_dir}};


pub fn add(cmd: &AddCommand) -> Result<(), anyhow::Error> {
    let data_path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found. Run `todo init` to initialize"))?
        .join("todos.bin");

    let mut task_list = deserialize(&data_path)?;
    let current_time = chrono::Local::now().naive_local().to_string();
    let priority = match cmd.priority {
        Priority::High => "High",
        Priority::Medium => "Medium",
        Priority::Low => "Low",
    };

    let new_task = Task {
        id: task_list.next_id,
        title: cmd.title.clone(),
        priority: priority.to_string(),
        completed: cmd.complete,
        due: cmd.due.to_string(),
        created_at: current_time.clone(),
    };
    task_list.next_id += 1;
    task_list.tasks.push(new_task);

    let mut file_path = File::create(&data_path)?;
    serialize(&mut file_path, &task_list)?;

    println!("{}", "New Task".bold());
    let status = if cmd.complete {
        "Done".green()
    } else {
        "Pending".red()
    };
    println!(
        "ID: {} | Title: {} | Status: {} | Priority: {} | Due: {} | CreatedAt: {}",
        (task_list.next_id - 1).to_string().cyan(),
        cmd.title,
        status,
        priority.to_string(),
        cmd.due.to_string(),
        current_time
    );
    println!("{}", "New Task Added Successfully!".green());
    Ok(())

}