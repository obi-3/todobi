use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

mod input;
mod menu;
mod todo;

fn get_path_by_env() -> Result<PathBuf, anyhow::Error> {
    let todo_dir = env::var("TODO_DIR").with_context(|| "Failed to read TODO_DIR.")?;
    let todo_dir = todo_dir
        .parse::<PathBuf>()
        .with_context(|| "Failed to parse TODO_DIR path")?;
    let file_path = todo_dir.join("todos.json");
    Ok(file_path)
}

#[derive(Debug, Serialize, Deserialize)]
struct Todobi {
    todos: Vec<todo::TodoBuilder>,
}

impl Todobi {
    fn new() -> Self {
        Self { todos: Vec::new() }
    }

    fn read_todos(&mut self, file_path: &PathBuf) -> anyhow::Result<()> {
        let todo_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;
        let reader = BufReader::new(todo_file);
        let todos: Vec<todo::TodoBuilder> = match serde_json::from_reader(reader) {
            Ok(todos) => todos,
            Err(_) => Vec::new(),
        };
        self.todos = todos;
        Ok(())
    }

    fn write_todos(&self, file_path: &PathBuf) -> anyhow::Result<()> {
        let todo_file = File::create(file_path)?;
        let writer = BufWriter::new(todo_file);
        serde_json::to_writer_pretty(writer, &self.todos)?;

        Ok(())
    }

    fn add_todo(&mut self) -> anyhow::Result<()> {
        let todo = input::input_todo(&console::Term::stdout())?;
        self.todos.push(todo);
        self.todos.sort();
        Ok(())
    }

    fn display_menu(&mut self) -> anyhow::Result<()> {
        self.todos.sort();
        let mut menu = menu::Menu::new(console::Term::stdout(), self.todos.to_owned());
        menu.select()?;
        self.todos = menu.todos;
        Ok(())
    }

    fn clear_dones(&mut self) {
        let todos: Vec<todo::TodoBuilder> = self
            .todos
            .to_owned()
            .into_iter()
            .filter(|todo| !todo.is_done())
            .collect();
        self.todos = todos;
    }
}

fn main() -> anyhow::Result<()> {
    let file_path = get_path_by_env()?;
    let mut todos = Todobi::new();
    todos.read_todos(&file_path)?;
    todos.display_menu()?;
    // todos.clear_dones();
    todos.write_todos(&file_path)?;

    Ok(())
}
