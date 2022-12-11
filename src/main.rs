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
    //! Get todo-list directory by environments.
    let todo_dir = env::var("TODO_DIR").with_context(|| "Failed to read TODO_DIR.")?;
    let todo_dir = todo_dir
        .parse::<PathBuf>()
        .with_context(|| "Failed to parse TODO_DIR path")?;
    let file_path = todo_dir.join("todos.json");
    Ok(file_path)
}

fn todobi() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let mut todos = todobi::Todobi::new();

    let file_path = get_path_by_env()?;
    todos.read(&file_path)?;
    match cli.command {
        cli::Commands::Edit => {
            todos.edit()?;
        }
        cli::Commands::Add { content, date } => {
            todos.add(content, date)?;
        }
        cli::Commands::Clear => {
            todos.clear();
        }
        cli::Commands::Show => {
            todos.show();
        }
    }
    todos.write(&file_path)?;

    Ok(())
}

fn main(){
    if let Err(e) = todobi() {
        println!("{e}");
    }
}
