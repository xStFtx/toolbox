mod organize;
mod git_helper;
mod todo;
mod bulk_rename;
mod dir_size;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "toolbox")]
#[command(about = "A multi-tool CLI utility written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Organize files in a directory
    Organize {
        #[arg(short, long, default_value = ".")]
        dir: String,
        #[arg(short, long, default_value = "type")]
        mode: String, // type, date, custom
    },
    /// Git helper commands
    GitHelper {
        #[arg(short, long, default_value = ".")]
        repo: String,
        #[arg(short, long)]
        clean_branches: bool,
        #[arg(short, long)]
        summary: bool,
    },
    /// Manage a todo list
    Todo {
        #[command(subcommand)]
        action: TodoAction,
    },
    /// Bulk rename files
    BulkRename {
        #[arg(short, long, default_value = ".")]
        dir: String,
        #[arg(short, long)]
        pattern: String,
        #[arg(short, long)]
        replace: String,
    },
    /// Analyze directory size
    DirSize {
        #[arg(short, long, default_value = ".")]
        dir: String,
        #[arg(short, long, default_value_t = 10)]
        top: usize,
    },
}

#[derive(Subcommand)]
enum TodoAction {
    /// Add a new todo item
    Add {
        text: String,
    },
    /// List all todo items
    List,
    /// Mark a todo item as done
    Done {
        id: usize,
    },
    /// Remove a todo item
    Remove {
        id: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Organize { dir, mode } => {
            organize::run(&dir, &mode);
        }
        Commands::GitHelper { repo, clean_branches, summary } => {
            git_helper::run(&repo, clean_branches, summary);
        }
        Commands::Todo { action } => {
            todo::run(action);
        }
        Commands::BulkRename { dir, pattern, replace } => {
            bulk_rename::run(&dir, &pattern, &replace);
        }
        Commands::DirSize { dir, top } => {
            dir_size::run(&dir, top);
        }
    }
}
