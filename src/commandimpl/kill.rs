use std::fs;

use colored::Colorize;

use crate::utility::finddir::find_todo_dir;


pub fn kill() -> Result<(), anyhow::Error> {
    let path = find_todo_dir()
        .ok_or_else(|| anyhow::anyhow!("No todo found!"))?;

    println!("Removing file: {:?}", path);
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    println!("{}", "todo deleted Successfully".to_string().green());
    Ok(())
}