use std::fs::File;

use colored::Colorize;

use crate::{arguments::{EditCommand, Priority}, utility::{conjucture::{deserialize, serialize}, finddir::find_todo_dir}};


pub fn edit(cmd: &EditCommand) -> Result<(), anyhow::Error> {
    let data_path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found. Run `todo init` to initialize"))?
        .join("todos.bin");
    let mut task_list = deserialize(&data_path)?;

    if let Some(item) = task_list.tasks.iter_mut().find(|item| item.id == cmd.id) {
        // title
        if let Some(title) = &cmd.title {
            item.title = title.clone();
        }
        // priority
        if let Some(prty) = &cmd.priority {
            item.priority = match prty {
                Priority::High => "High".to_string(),
                Priority::Medium => "Medium".to_string(),
                Priority::Low => "Low".to_string(),
            };
        }
        // due
        if let Some(due) = &cmd.due {
            item.due = due.to_string();
        }
        // complete
        if let Some(cpt) = &cmd.complete {
            item.completed = *cpt;
        }
    }
    let mut file_path = File::create(&data_path)?;
    serialize(&mut file_path, &task_list)?;
    println!("{}", "Task edited successfully".green());
    Ok(())
}