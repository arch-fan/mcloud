mod commands;
pub mod movistar;
pub mod utils;

use clap::Parser;
use commands::{cli_handler, Cli};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    cli_handler(&cli).await;
}
