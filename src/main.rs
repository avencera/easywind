// all functionality only used by the CLI
pub mod cli;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use eyre::Result;

#[derive(Debug, Parser)]
#[command(display_name = "EasyWind", author, version, about)]
#[command(arg_required_else_help(true))]
#[command(styles=cli::get_styles())]
pub struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start the server and tailwind watcher
    #[command(visible_aliases = ["s"])]
    Start(StartArgs),

    /// Run a live reloading server to serve content
    #[command(name = "serve", visible_aliases = ["server"])]
    Server(ServerArgs),

    /// Run the tailwind watcher that generates the CSS
    #[command(visible_aliases = ["t"])]
    Tailwind(TailwindArgs),
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct StartArgs {
    #[clap(default_value = ".")]
    pub root_dir: PathBuf,

    /// Port the server shoud use, defaults to 3500
    #[clap(short, long, default_value = "3500")]
    pub port: u16,

    /// Open in your browser
    #[clap(short = 'O', long)]
    pub open: bool,

    /// Input css file to process
    #[clap(short, long, default_value = "src/styles.css")]
    pub input: PathBuf,

    /// Where you want the final CSS file to be written
    #[clap(short, long, default_value = "dest/app.css")]
    pub output: PathBuf,
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct ServerArgs {
    #[clap(default_value = ".")]
    pub root_dir: PathBuf,

    /// Port the server shoud use, defaults to 3500
    #[clap(short, long, default_value = "3500")]
    pub port: u16,

    /// Open in your browser
    #[clap(short, long)]
    pub open: bool,
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct TailwindArgs {
    /// Path to the root directory of the project. This is where the `tailwind.config.js` file is located.
    ///
    /// Defaults to the current directory
    #[clap(default_value = ".")]
    pub root_dir: PathBuf,

    /// Input css file to process
    #[clap(short, long, default_value = "src/styles.css")]
    pub input: PathBuf,

    /// Where you want the final CSS file to be written
    #[clap(short, long, default_value = "dest/app.css")]
    pub output: PathBuf,

    /// Watch for changes in input CSS and recompile the output CSS
    #[clap(short, long)]
    pub watch: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logging
    pretty_env_logger::init();
    let cli = CliArgs::parse();

    match cli {
        CliArgs {
            command: Commands::Start(args),
        } => {
            easywind::start::start(args.into()).await?;
        }
        CliArgs {
            command: Commands::Server(args),
        } => {
            easywind::server::start(args.into()).await?;
        }
        CliArgs {
            command: Commands::Tailwind(args),
        } => {
            easywind::tailwind::start(args.into())?;
        }
    }

    Ok(())
}
