use crate::input::input_todo;
use crate::todo;
use anyhow;
use console::{Key, Term};

#[derive(Debug, PartialEq)]
enum MenuKind {
    Todo,
    Done,
    All,
}

#[derive(Debug)]
pub struct Menu {
    term: Term,
    menu: MenuKind,
    pub todos: Vec<todo::TodoBuilder>,
    cursor: usize,
    curr_lines: usize,
}

impl Menu {
    pub fn new(term: Term, todos: Vec<todo::TodoBuilder>) -> Self {
        let cursor = 1;
        Self {
            term,
            menu: MenuKind::Todo,
            todos,
            cursor,
            curr_lines: 0,
        }
    }

    pub fn select(&mut self) -> anyhow::Result<()> {
        self.term.hide_cursor()?;
        loop {
            self.print_todos()?;
            let is_noline = self.curr_lines == 0;
            match (is_noline, self.term.read_key()?) {
                (_, Key::Escape | Key::Char('q')) => {
                    self.clear()?;
                    break;
                }
                (_, Key::Char('a')) => {
                    self.clear()?;
                    let todo = input_todo(&self.term)?;
                    self.term.hide_cursor()?;
                    self.todos.push(todo);
                    self.todos.sort();
                    continue;
                }
                (_, Key::ArrowLeft | Key::Char('l')) => {
                    self.next_menu();
                    self.cursor = 1;
                }
                (_, Key::ArrowRight | Key::Char('h')) => {
                    self.prev_menu();
                    self.cursor = 1;
                }
                (false, Key::ArrowUp | Key::BackTab | Key::Char('k')) => {
                    if self.cursor == 1 {
                        self.cursor = self.curr_lines;
                    } else {
                        self.cursor -= 1;
                    }
                }
                (false, Key::ArrowDown | Key::Tab | Key::Char('j')) => {
                    if self.cursor == self.curr_lines {
                        self.cursor = 1;
                    } else {
                        self.cursor += 1;
                    }
                }
                (false, Key::Enter) => {
                    self.toggle_todo();
                    if self.menu != MenuKind::All && self.cursor == self.curr_lines {
                        self.cursor -= 1;
                    }
                }
                _ => {}
            }
            self.clear()?;
        }
        self.term.show_cursor()?;
        Ok(())
    }

    fn next_menu(&mut self) {
        use MenuKind::*;
        self.menu = match self.menu {
            Todo => Done,
            Done => All,
            All => Todo,
        }
    }

    fn prev_menu(&mut self) {
        use MenuKind::*;
        self.menu = match self.menu {
            Todo => All,
            Done => Todo,
            All => Done,
        }
    }

    fn print_todos(&mut self) -> anyhow::Result<()> {
        let title = match self.menu {
            MenuKind::Todo => format!("{}|Done|All", console::style("Todo").underlined()),
            MenuKind::Done => format!("Todo|{}|All", console::style("Done").underlined()),
            MenuKind::All => format!("Todo|Done|{}", console::style("All").underlined()),
        };

        self.term.write_line(&title)?;
        self.curr_lines = 0;
        for todo in self.todos.iter() {
            match (&self.menu, todo.is_done()) {
                (MenuKind::Todo, false) | (MenuKind::Done, true) | (MenuKind::All, _) => {}
                _ => continue,
            }
            self.curr_lines += 1;
            let indicator = if self.cursor == self.curr_lines {
                ">"
            } else {
                " "
            };
            let done_sign = if todo.is_done() { "x" } else { " " };
            self.term.write_line(
                format!("{} [{}] {}", indicator, done_sign, todo.to_string()).as_str(),
            )?;
        }
        Ok(())
    }

    fn clear(&mut self) -> anyhow::Result<()> {
        self.term.clear_last_lines(self.curr_lines + 1)?;
        Ok(())
    }

    fn toggle_todo(&mut self) {
        let mut cnt = 0;
        let todos: Vec<todo::TodoBuilder> = self
            .todos
            .clone()
            .into_iter()
            .map(|todo| {
                match (&self.menu, todo.is_done()) {
                    (MenuKind::Todo, false) | (MenuKind::Done, true) | (MenuKind::All, _) => {
                        cnt += 1;
                        if cnt == self.cursor {
                            return todo.toggle();
                        }
                    }
                    _ => {}
                }
                todo
            })
            .collect();
        self.todos = todos;
    }
}
