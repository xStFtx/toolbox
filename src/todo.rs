use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use clap::Subcommand;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub text: String,
    pub done: bool,
    pub priority: Priority,
    pub category: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum TodoAction {
    /// Add a new todo item
    Add {
        text: String,
        #[arg(short, long, default_value = "Medium")]
        priority: String,
        #[arg(short, long)]
        category: Option<String>,
    },
    /// List all todo items
    List {
        #[arg(short, long)]
        priority: Option<String>,
        #[arg(short, long)]
        category: Option<String>,
    },
    /// Mark a todo item as done
    Done {
        id: usize,
    },
    /// Remove a todo item
    Remove {
        id: usize,
    },
}

fn get_todo_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("toolbox_todo.json");
    path
}

pub fn run(action: TodoAction) {
    let path = get_todo_path();
    let mut todos = load_todos(&path);
    match action {
        TodoAction::Add { text, priority, category } => {
            let priority = match_priority(&priority);
            todos.push(TodoItem { text, done: false, priority, category });
            save_todos(&path, &todos);
            println!("Added todo.");
        }
        TodoAction::List { priority, category } => {
            let filter_priority = priority.as_ref().map(|p| match_priority(p));
            for (i, todo) in todos.iter().enumerate() {
                if let Some(ref p) = filter_priority {
                    if &todo.priority != p { continue; }
                }
                if let Some(ref cat) = category {
                    if todo.category.as_deref() != Some(cat.as_str()) { continue; }
                }
                let status = if todo.done { "[x]" } else { "[ ]" };
                let cat_disp = todo.category.as_deref().unwrap_or("-");
                println!("{} {} {} (Priority: {:?}, Category: {})", i, status, todo.text, todo.priority, cat_disp);
            }
        }
        TodoAction::Done { id } => {
            if let Some(todo) = todos.get_mut(id) {
                todo.done = true;
                save_todos(&path, &todos);
                println!("Marked as done.");
            } else {
                println!("Invalid ID");
            }
        }
        TodoAction::Remove { id } => {
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

fn match_priority(s: &str) -> Priority {
    match s.to_lowercase().as_str() {
        "low" => Priority::Low,
        "high" => Priority::High,
        _ => Priority::Medium,
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
