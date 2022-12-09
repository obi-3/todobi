use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add todo
    Add {
        #[arg(short, long)]
        content: Option<String>,
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Clear all todo which are done
    Clear,
    /// Show all todo
    Show,
}
