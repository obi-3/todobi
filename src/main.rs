// use chrono::prelude::*;
// use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

mod input;
mod menu;
mod todo;

#[derive(Debug, Serialize, Deserialize)]
struct Todobi {
    todos: Vec<todo::TodoBuilder>,
}

fn read_todolist() -> anyhow::Result<()> {
    let Ok(val) = env::var("TODO_DIR") else {
        println!("Couldn't read TODO_DIR variable in your environment.");
        println!("If you don't set TODO_DIR, Please set path which register todo-files.");
        std::process::exit(1);
    };
    let todo_dir: PathBuf = val.parse()?;
    let list_path = todo_dir.join("todolist.json");
    println!("path: {list_path:?}");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // let local: DateTime<Local> = Local::now();
    // let dt = local + Duration::weeks(4);
    // println!("{:?}", dt);
    // println!("{:?}", dt - local);
    // println!("{}", (dt - local).num_days());
    // let todo = todo::TodoBuilder::new().title("test").desc("hello world");
    // println!("{todo:?}");
    //
    // read_todolist()?;

    // println!("{:?}", todo);
    //
    menu::display_menu()?;

    Ok(())
}
