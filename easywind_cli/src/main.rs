// all functionality only used by the CLI
pub mod cli;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use eyre::Result;
use pretty_env_logger::env_logger::Env;

#[derive(Debug, Parser)]
#[command(display_name = "EasyWind", author, version)]
#[command(arg_required_else_help(true))]
#[command(styles=cli::get_styles())]
pub struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize a new project
    #[command(visible_aliases = ["new", "i"])]
    Init(InitArgs),

    /// Start the server and tailwind watcher
    #[command(visible_aliases = ["run", "s"])]
    Start(StartArgs),

    /// Run a live reloading server to serve content
    #[command(name = "serve")]
    Server(ServerArgs),

    /// Run the tailwind watcher that generates the CSS
    #[command(visible_aliases = ["t"])]
    Tailwind(TailwindArgs),
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct InitArgs {
    /// Name of the project to initialize
    ///
    /// This will be used to create a directory with the same name
    /// (usage: easywind init portfolio)
    pub project_name: String,
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
    #[clap(short, long)]
    pub input: Option<PathBuf>,

    /// Where you want the final CSS file to be written
    #[clap(short, long)]
    pub output: Option<PathBuf>,
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
    #[clap(short, long, default_value = "src/app.css")]
    pub input: PathBuf,

    /// Where you want the final CSS file to be written
    #[clap(short, long, default_value = "dist/app.css")]
    pub output: PathBuf,

    /// Watch for changes in input CSS and recompile the output CSS
    #[clap(short, long)]
    pub watch: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logging
    let env = Env::new().filter_or("LOG_LEVEL", "info");
    pretty_env_logger::env_logger::init_from_env(env);

    // pretty errors
    color_eyre::install()?;

    let cli = CliArgs::parse();

    match cli {
        CliArgs {
            command: Commands::Init(args),
        } => {
            easywind::init::run(args.into())?;
        }

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
