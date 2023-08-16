use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

use eyre::{eyre, Context, Result};

use super::TailwindArgs;

pub fn watch(args: TailwindArgs) -> Result<()> {
    let mut tailwind_args = base_args(&args)?;
    tailwind_args.push("--watch");

    tailwind(&tailwind_args, &args.root_dir).wrap_err("failed to run tailwind")?;

    Ok(())
}

pub fn build(args: TailwindArgs) -> Result<()> {
    let tailwind_args = base_args(&args)?;
    tailwind(&tailwind_args, &args.root_dir).wrap_err("failed to run tailwind")?;

    Ok(())
}

fn base_args(args: &TailwindArgs) -> Result<Vec<&str>> {
    let config_file = args.root_dir.join("tailwind.config.js");

    if !config_file.exists() {
        return Err(eyre!(
            "tailwind.config.js does not exist in {}",
            args.root_dir.to_string_lossy()
        ));
    }

    let base_args = vec![
        "--input",
        args.input
            .to_str()
            .ok_or_else(|| eyre!("input path is not valid utf-8"))?,
        "--output",
        args.output
            .to_str()
            .ok_or_else(|| eyre!("output path is not valid utf-8"))?,
    ];

    Ok(base_args)
}

pub fn tailwind(args: &[&str], root_dir: &PathBuf) -> Result<(), std::io::Error> {
    let reader = duct::cmd("npx", ["tailwind"].iter().chain(args))
        .stderr_to_stdout()
        .stdout_capture()
        .dir(root_dir)
        .reader()?;

    for line in BufReader::new(reader).lines() {
        log::info!("{}", line?);
    }

    Ok(())
}
