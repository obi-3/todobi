use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    },
    /// Clear all todo which are done
    Clear {
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    },
    /// Show all todo
    Show {
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    },
}
