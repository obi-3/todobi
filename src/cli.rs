use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add task
    Add {
        content: Option<String>,
        date: Option<String>,
    },
    /// Clear all tasks which are done
    Clear,
    /// Edit todo-list
    Edit,
    /// Show all todo-list
    Show,
}
