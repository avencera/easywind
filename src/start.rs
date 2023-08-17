use eyre::Result;
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

impl From<StartArgs> for crate::tailwind::TailwindArgs {
    fn from(args: StartArgs) -> Self {
        let root_dir = args.root_dir;
        let input = args.input.unwrap_or_else(|| root_dir.join("src/app.css"));
        let output = args.output.unwrap_or_else(|| root_dir.join("dist/app.css"));

        Self {
            root_dir,
            input: std::fs::canonicalize(input).expect("Unable to canonicalize input file"),
            output: std::fs::canonicalize(output).expect("Unable to canonicalize output file"),
            watch: true,
        }
    }
}

pub async fn start(args: StartArgs) -> Result<()> {
    let server_args: crate::server::ServerArgs = args.clone().into();
    let server_task = tokio::task::spawn(async move { crate::server::start(server_args).await });

    let tailwind_args: crate::tailwind::TailwindArgs = args.into();
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
