use color_eyre::Help;
use eyre::{Context, Result};
use futures::{stream::FuturesUnordered, StreamExt};
use std::path::PathBuf;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct StartArgs {
    pub root_dir: PathBuf,
    pub port: u16,
    pub open: bool,
    pub input: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

impl From<StartArgs> for crate::server::ServerArgs {
    fn from(args: StartArgs) -> Self {
        Self {
            root_dir: args.root_dir,
            port: args.port,
            open: args.open,
        }
    }
}

impl TryFrom<StartArgs> for crate::tailwind::TailwindArgs {
    type Error = color_eyre::Report;

    fn try_from(args: StartArgs) -> Result<Self, Self::Error> {
        let root_dir = args.root_dir;
        let input = args.input.unwrap_or_else(|| root_dir.join("src/app.css"));
        let output = args.output.unwrap_or_else(|| root_dir.join("dist/app.css"));

        let input = std::fs::canonicalize(&input)
            .wrap_err_with(|| format!("Unable to find input file: {}", input.to_string_lossy()))
            .suggestion("Try running `easywind init` to create a new project")
            .suggestion("Try setting the location of your input file with `--input` flag")?;

        if let Err(_) = std::fs::canonicalize(&output) {
            std::fs::write(&output, "")
                .wrap_err_with(|| {
                    format!(
                        "Unable to create empty output file at: {}",
                        output.to_string_lossy()
                    )
                })
                .suggestion("Make sure the directory of the output file exists")
                .suggestion("Try setting a different output file location with `--output` flag")?;
        }

        Ok(Self {
            root_dir,
            input,
            output,
            watch: true,
        })
    }
}

pub async fn start(args: StartArgs) -> Result<()> {
    let server_args: crate::server::ServerArgs = args.clone().into();
    let server_task = tokio::task::spawn(async move { crate::server::start(server_args).await });

    let tailwind_args: crate::tailwind::TailwindArgs = args.try_into()?;
    let tailwind_task = tokio::task::spawn_blocking(|| crate::tailwind::start(tailwind_args));

    let tasks = vec![tailwind_task, server_task];

    let mut futures = tasks
        .into_iter()
        .collect::<FuturesUnordered<JoinHandle<_>>>();

    // return on first errror
    if let Some(Ok(Err(err))) = futures.next().await {
        return Err(err);
    };

    Ok(())
}
