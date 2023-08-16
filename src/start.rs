use std::path::PathBuf;

use eyre::Result;

#[derive(Debug, Clone)]
pub struct StartArgs {
    pub root_dir: PathBuf,
    pub port: u16,
    pub open: bool,
    pub input: PathBuf,
    pub output: PathBuf,
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
        Self {
            root_dir: args.root_dir,
            input: args.input,
            output: args.output,
            watch: true,
        }
    }
}

pub async fn start(args: StartArgs) -> Result<()> {
    let server_args: crate::server::ServerArgs = args.clone().into();
    let server_task = tokio::task::spawn(async move { crate::server::start(server_args).await });

    let tailwind_args: crate::tailwind::TailwindArgs = args.into();
    let tailwind_task = tokio::task::spawn(async move { crate::tailwind::start(tailwind_args) });

    let _ = tokio::try_join!(server_task, tailwind_task)?;

    Ok(())
}
