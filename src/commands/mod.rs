mod setkey;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Log in to Movistar Cloud
    Setkey {
        /// Validation Key of Movistar Cloud
        key: String,
    },
}

pub async fn cli_handler(cli: &Cli) {
    // Subcommands handling
    match &cli.command {
        Some(Commands::Setkey { key }) => setkey::handler(key).await,
        None => {}
    }
}
