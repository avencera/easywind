use std::io::{BufRead, BufReader};

use eyre::{eyre, Context, Result};

use super::TailwindArgs;

pub fn init() -> Result<()> {
    tailwind(&["init"])?;
    Ok(())
}

pub fn watch(args: TailwindArgs) -> Result<()> {
    tailwind(&[
        "--input",
        args.input
            .to_str()
            .ok_or_else(|| eyre!("input path is not valid utf-8"))?,
        "--output",
        args.output
            .to_str()
            .ok_or_else(|| eyre!("output path is not valid utf-8"))?,
        "--watch",
    ])
    .wrap_err("failed to run tailwind")?;

    Ok(())
}

pub fn build(args: TailwindArgs) -> Result<()> {
    tailwind(&[
        "--input",
        args.input
            .to_str()
            .ok_or_else(|| eyre!("input path is not valid utf-8"))?,
        "--output",
        args.output
            .to_str()
            .ok_or_else(|| eyre!("output path is not valid utf-8"))?,
    ])
    .wrap_err("failed to run tailwind")?;

    Ok(())
}

pub fn tailwind(args: &[&str]) -> Result<(), std::io::Error> {
    let reader = duct::cmd("npx", ["tailwind"].iter().chain(args))
        .stderr_to_stdout()
        .stdout_capture()
        .reader()?;

    for line in BufReader::new(reader).lines() {
        log::info!("{}", line?);
    }

    Ok(())
}
