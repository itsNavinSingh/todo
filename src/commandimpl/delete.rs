use std::fs::File;

use colored::Colorize;

use crate::{arguments::DeleteCommand, utility::{conjucture::{deserialize, serialize}, finddir::find_todo_dir}};

pub fn delete(cmd: &DeleteCommand) -> Result<(), anyhow::Error> {
    let data_path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found. Run `todo init` to initialize"))?
        .join("todos.bin");

    let mut task_list = deserialize(&data_path)?;
    if !task_list.tasks.iter().any(|item| item.id == cmd.id) {
        println!("{}: No task with id: {} exists", "WARNING".yellow(), cmd.id);
        return Ok(());
    }
    task_list.tasks = task_list.tasks.into_iter().filter(|x| x.id != cmd.id).collect();
    
    let mut file_path = File::create(&data_path)?;
    serialize(&mut file_path, &task_list)?;
    println!("{}", "Task Deleted Successfully!".green());
    Ok(())
}