mod organize;
mod git_helper;
mod todo;
mod bulk_rename;
mod dir_size;

use clap::{Parser, Subcommand};
use colored::*;
use std::io::{self, Write};
use shlex;

#[derive(Parser)]
#[command(name = "toolbox")]
#[command(about = "A multi-tool CLI utility written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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
        action: todo::TodoAction,
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
    /// Interactive mode
    Interactive,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Organize { dir, mode }) => {
            organize::run(&dir, &mode);
        }
        Some(Commands::GitHelper { repo, clean_branches, summary }) => {
            git_helper::run(&repo, clean_branches, summary);
        }
        Some(Commands::Todo { action }) => {
            todo::run(action);
        }
        Some(Commands::BulkRename { dir, pattern, replace }) => {
            bulk_rename::run(&dir, &pattern, &replace);
        }
        Some(Commands::DirSize { dir, top }) => {
            dir_size::run(&dir, top);
        }
        Some(Commands::Interactive) | None => {
            println!("{}", "Toolbox Interactive Mode".bold().cyan());
            println!("Type a command (or 'help', 'exit'):");
            interactive_shell();
        }
    }
}

fn interactive_shell() {
    let mut input = String::new();
    loop {
        print!("{} ", "toolbox>".green().bold());
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", "Failed to read input".red());
            continue;
        }
        let trimmed = input.trim();
        if trimmed == "exit" || trimmed == "quit" { break; }
        if trimmed == "help" {
            println!("Available commands: organize, git-helper, todo, bulk-rename, dir-size, exit");
            continue;
        }
        let args = shlex::split(trimmed).unwrap_or_default();
        let mut full_args = vec!["toolbox".to_string()];
        full_args.extend(args);
        match Cli::try_parse_from(&full_args) {
            Ok(cli) => match cli.command {
                Some(Commands::Organize { dir, mode }) => organize::run(&dir, &mode),
                Some(Commands::GitHelper { repo, clean_branches, summary }) => git_helper::run(&repo, clean_branches, summary),
                Some(Commands::Todo { action }) => todo::run(action),
                Some(Commands::BulkRename { dir, pattern, replace }) => bulk_rename::run(&dir, &pattern, &replace),
                Some(Commands::DirSize { dir, top }) => dir_size::run(&dir, top),
                Some(Commands::Interactive) | None => {},
            },
            Err(e) => println!("{}", format!("Error: {}", e).red()),
        }
    }
    println!("{}", "Goodbye!".yellow());
}
