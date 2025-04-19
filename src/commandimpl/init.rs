use std::fs::{self, File};

use anyhow::Context;
use colored::Colorize;
use crate::utility::conjucture::serialize;
use crate::tasks::TaskList;


pub fn init() -> Result<(), anyhow::Error> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let todo_dir = current_dir.join(".todo");
    let data_path = todo_dir.join("todos.bin");

    if todo_dir.exists() {
        eprintln!("{}","Todo list already initialized.".red());
        return Err(anyhow::anyhow!("Directory already exists"));
    }

    if data_path.exists() {
        eprintln!("{}", "todo already exist".red());
        eprintln!("{}", "Use 'todo remove' to remove todo from this project and run the command again".red());
        return Err(anyhow::anyhow!("Project already exist"));
    }

    fs::create_dir(&todo_dir).context("Failed to create .todo directory")?;

    let task_list = TaskList {
        tasks: Vec::new(),
        next_id: 1,
    };

    let mut file = File::create(&data_path).context("Failed to create todos.bin")?;

    serialize(&mut file, &task_list)?;
    println!("{}", "initialized empty todo".green());
    println!("Location: {:?}", data_path);
    Ok(())

}