use crate::{input, menu, todo};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
#[derive(Debug, Serialize, Deserialize)]
pub struct Todobi {
    pub todos: Vec<todo::TodoBuilder>,
}

impl Todobi {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    pub fn read_todos(&mut self, file_path: &PathBuf) -> anyhow::Result<()> {
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

    pub fn write_todos(&self, file_path: &PathBuf) -> anyhow::Result<()> {
        let todo_file = File::create(file_path)?;
        let writer = BufWriter::new(todo_file);
        serde_json::to_writer_pretty(writer, &self.todos)?;

        Ok(())
    }

    pub fn add_todo(&mut self) -> anyhow::Result<()> {
        let todo = input::input_todo(&console::Term::stdout())?;
        self.todos.push(todo);
        self.todos.sort();
        Ok(())
    }

    pub fn display_menu(&mut self) -> anyhow::Result<()> {
        self.todos.sort();
        let mut menu = menu::Menu::new(console::Term::stdout(), self.todos.to_owned());
        menu.select()?;
        self.todos = menu.todos;
        Ok(())
    }

    pub fn clear_dones(&mut self) {
        let todos: Vec<todo::TodoBuilder> = self
            .todos
            .to_owned()
            .into_iter()
            .filter(|todo| !todo.is_done())
            .collect();
        self.todos = todos;
    }
}
