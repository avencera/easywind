use std::process::Output;

use eyre::{eyre, Context, Result};

use super::TailwindArgs;

pub fn init() -> Result<Output> {
    let output = tailwind(&["init"])?;
    Ok(output)
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

pub fn tailwind(args: &[&str]) -> Result<Output, std::io::Error> {
    duct::cmd("npx", ["tailwind"].iter().chain(args)).run()
}
