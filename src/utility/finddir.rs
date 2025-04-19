use std::path::PathBuf;

pub fn find_todo_dir() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    let mut current = current_dir.as_path();
    loop {
        let todo_dir = current.join(".todo");
        if todo_dir.exists() && todo_dir.is_dir() {
            return Some(todo_dir);
        }
        current = current.parent()?;
    }
}