use anyhow::Context;
use clap::Parser;
use std::env;
use std::path::PathBuf;

mod cli;
mod input;
mod menu;
mod todo;
mod todobi;

fn get_path_by_env() -> Result<PathBuf, anyhow::Error> {
    let todo_dir = env::var("TODO_DIR").with_context(|| "Failed to read TODO_DIR.")?;
    let todo_dir = todo_dir
        .parse::<PathBuf>()
        .with_context(|| "Failed to parse TODO_DIR path")?;
    let file_path = todo_dir.join("todos.json");
    Ok(file_path)
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let mut todos = todobi::Todobi::new();
    match cli.command {
        cli::Commands::Show => {
            let file_path = get_path_by_env()?;
            todos.read_todos(&file_path)?;
            todos.display_menu()?;
            todos.write_todos(&file_path)?;
        }
        cli::Commands::Add { .. } => {
            let file_path = get_path_by_env()?;
            todos.read_todos(&file_path)?;
            todos.add_todo()?;
            todos.write_todos(&file_path)?;
        }
        cli::Commands::Clear => {
            let file_path = get_path_by_env()?;
            todos.read_todos(&file_path)?;
            todos.clear_dones();
            todos.write_todos(&file_path)?;
        }
    }

    Ok(())
}
