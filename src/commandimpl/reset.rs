use std::fs::File;

use colored::Colorize;

use crate::utility::{conjucture::{deserialize, serialize}, finddir::find_todo_dir};


pub fn reset() -> Result<(), anyhow::Error> {
    let data_path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found. Run `todo init` to initialize"))?
        .join("todos.bin");

    let mut task_list = deserialize(&data_path)?;
    let mut idx: u32 = 1;
    for task in &mut task_list.tasks {
        task.id = idx;
        idx += 1;
    }
    task_list.next_id = idx;
    
    let mut file_path = File::create(&data_path)?;
    serialize(&mut file_path, &task_list)?;
    println!("{}", "Task Index Reseted Successfully".green());
    Ok(())
}