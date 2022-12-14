use crate::{input, menu, todo};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todobi {
    pub todos: Vec<todo::Todo>,
}

impl Todobi {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    pub fn read(&mut self, file_path: &PathBuf) -> anyhow::Result<()> {
        //! Read todo-list.
        let todo_list = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;
        let reader = BufReader::new(todo_list);
        let todos: Vec<todo::Todo> = match serde_json::from_reader(reader) {
            Ok(todos) => todos,
            Err(_) => Vec::new(),
        };
        self.todos = todos;
        Ok(())
    }

    pub fn write(&self, file_path: &PathBuf) -> anyhow::Result<()> {
        //! Overwrite todo-list.
        let todo_list = File::create(file_path)?;
        let writer = BufWriter::new(todo_list);
        serde_json::to_writer_pretty(writer, &self.todos)?;

        Ok(())
    }

    pub fn add(&mut self, content: Option<String>, date: Option<String>) -> anyhow::Result<()> {
        let todo = input::input_todo(&console::Term::stdout(), content, date)?;
        println!("Add:[ {} ]", todo.format(None));
        self.todos.push(todo);
        self.todos.sort();
        Ok(())
    }

    pub fn edit(&mut self) -> anyhow::Result<()> {
        self.todos.sort();
        let mut menu = menu::Menu::new(console::Term::stdout(), self.todos.to_owned());
        menu.select()?;
        self.todos = menu.todos;
        Ok(())
    }

    pub fn clear(&mut self) {
        let todos: Vec<todo::Todo> = self
            .todos
            .iter()
            .cloned()
            .filter(|todo| !todo.is_done())
            .collect();
        self.todos = todos;
    }

    pub fn show(&self) {
        let mut max = 0;
        for todo in &self.todos {
            let len = todo.get_content_len();
            if len > max {
                max = len;
            }
        }
        for todo in self.todos.iter() {
            if !todo.is_done() {
                println!("  {}", todo.format(Some(max)));
            }
        }
    }
}
