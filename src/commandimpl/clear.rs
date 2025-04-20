use std::fs::File;

use colored::Colorize;

use crate::utility::{conjucture::{deserialize, serialize}, finddir::find_todo_dir};


pub fn clear() -> Result<(), anyhow::Error> {
    let data_path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found. Run `todo init` to initialize"))?
        .join("todos.bin");
    let mut task_list = deserialize(&data_path)?;

    task_list.tasks = task_list.tasks.into_iter().filter(|item| !item.completed).collect();

    let mut file_path = File::create(&data_path)?;
    serialize(&mut file_path, &task_list)?;
    println!("{}", "All completed tasks deleted successfully".green());
    Ok(())
}