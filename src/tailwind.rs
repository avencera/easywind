pub mod cli;

use std::path::PathBuf;

use eyre::Result;

#[derive(Debug, Clone)]
pub struct TailwindArgs {
    pub root_dir: PathBuf,
    pub input: PathBuf,
    pub output: PathBuf,
    pub watch: bool,
}

pub fn start(args: TailwindArgs) -> Result<()> {
    if args.watch {
        self::cli::watch(args)?;
    } else {
        self::cli::build(args)?;
    }

    Ok(())
}
