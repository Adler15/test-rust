pub use clap::{Parser, Subcommand};
pub use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[arg(long, short, default_value = "2")]
    pub two: Option<String>,
    #[arg(long)]
    pub one: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// start server
    Start {
        /// Sets a custom config file
        #[arg(short, long, value_name = "console.yml")]
        config: Option<PathBuf>,
    },
}
