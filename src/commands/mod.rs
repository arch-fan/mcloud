use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Log in to Movistar Cloud
    login: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    list,
}
