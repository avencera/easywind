// all functionality only used by the CLI
pub mod cli;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use easywind::server;
use eyre::Result;

#[derive(Debug, Parser)]
#[command(display_name = "EasyWind", author, version, about)]
#[command(arg_required_else_help(true))]
pub struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run a live reloading server to serve tailwind content
    #[command(name = "serve", visible_aliases = ["s", "server"])]
    Server(ServerArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct ServerArgs {
    #[clap(default_value = ".")]
    pub root_dir: PathBuf,

    #[clap(short, long, default_value = "3000")]
    pub port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logging
    pretty_env_logger::init();
    let cli = CliArgs::parse();

    match cli {
        CliArgs {
            command: Commands::Server(ServerArgs { root_dir, port }),
        } => {
            server::start(root_dir, port).await?;
        }
    }

    Ok(())
}
