use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    text: String,
    done: bool,
}

fn get_todo_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("toolbox_todo.json");
    path
}

pub fn run(action: crate::TodoAction) {
    let path = get_todo_path();
    let mut todos = load_todos(&path);
    match action {
        crate::TodoAction::Add { text } => {
            todos.push(TodoItem { text, done: false });
            save_todos(&path, &todos);
            println!("Added todo.");
        }
        crate::TodoAction::List => {
            for (i, todo) in todos.iter().enumerate() {
                let status = if todo.done { "[x]" } else { "[ ]" };
                println!("{} {} {}", i, status, todo.text);
            }
        }
        crate::TodoAction::Done { id } => {
            if let Some(todo) = todos.get_mut(id) {
                todo.done = true;
                save_todos(&path, &todos);
                println!("Marked as done.");
            } else {
                println!("Invalid ID");
            }
        }
        crate::TodoAction::Remove { id } => {
            if id < todos.len() {
                todos.remove(id);
                save_todos(&path, &todos);
                println!("Removed todo.");
            } else {
                println!("Invalid ID");
            }
        }
    }
}

fn load_todos(path: &PathBuf) -> Vec<TodoItem> {
    if let Ok(file) = fs::File::open(path) {
        serde_json::from_reader(file).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_todos(path: &PathBuf, todos: &[TodoItem]) {
    if let Ok(file) = fs::File::create(path) {
        serde_json::to_writer_pretty(file, todos).unwrap();
    }
}
