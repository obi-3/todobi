use crate::input::input_todo;
use crate::todo;
// use anyhow;
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
    line_count: usize,
}

impl Menu {
    pub fn new(term: Term, todos: Vec<todo::TodoBuilder>) -> Self {
        Self {
            term,
            menu: MenuKind::Todo,
            todos,
            cursor: 1,
            line_count: 0, // count of displayed lines
        }
    }

    pub fn select(&mut self) -> anyhow::Result<()> {
        self.term.hide_cursor()?;
        loop {
            self.print_todos()?;
            let is_noline = self.line_count == 0;
            match (is_noline, self.term.read_key()?) {
                (_, Key::Escape | Key::Char('q')) => {
                    self.clear()?;
                    break;
                }
                (_, Key::Char('a')) => {
                    self.clear()?;
                    let todo = input_todo(&self.term, None, None)?;
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
                        self.cursor = self.line_count;
                    } else {
                        self.cursor -= 1;
                    }
                }
                (false, Key::ArrowDown | Key::Tab | Key::Char('j')) => {
                    if self.cursor == self.line_count {
                        self.cursor = 1;
                    } else {
                        self.cursor += 1;
                    }
                }
                (false, Key::Enter) => {
                    self.toggle_todo();
                    if self.menu != MenuKind::All && self.cursor == self.line_count {
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
        self.line_count = 0;
        for todo in self.todos.iter() {
            if !is_displayed_todo(&self.menu, todo) {
                continue;
            }
            self.line_count += 1;
            let indicator = if self.cursor == self.line_count {
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
        let header = 1;
        self.term.clear_last_lines(self.line_count + header)?;
        Ok(())
    }

    fn toggle_todo(&mut self) {
        let mut cnt = 0;
        let todos: Vec<todo::TodoBuilder> = self
            .todos
            .clone()
            .into_iter()
            .map(|todo| {
                if is_displayed_todo(&self.menu, &todo) {
                    cnt += 1;
                    if cnt == self.cursor {
                        return todo.toggle();
                    }
                }
                todo
            })
            .collect();
        self.todos = todos;
    }
}

fn is_displayed_todo(menu_kind: &MenuKind, todo: &todo::TodoBuilder) -> bool {
    matches!(
        (menu_kind, todo.is_done()),
        (MenuKind::Todo, false) | (MenuKind::Done, true) | (MenuKind::All, _)
    )
}
