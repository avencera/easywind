pub mod cli;

use std::path::PathBuf;

use eyre::{eyre, Result};

#[derive(Debug, Clone)]
pub struct TailwindArgs {
    pub root_dir: PathBuf,
    pub input: PathBuf,
    pub output: PathBuf,
    pub watch: bool,
}

pub fn start(args: TailwindArgs) -> Result<()> {
    if args.input == args.output {
        return Err(eyre!("input and output files cannot be the same"));
    }

    if !args.input.exists() {
        return Err(eyre!(
            "input file ({}) does not exist",
            args.input.to_str().unwrap_or_default()
        ));
    }

    if args.watch {
        self::cli::watch(args)?;
    } else {
        self::cli::build(args)?;
    }

    Ok(())
}
