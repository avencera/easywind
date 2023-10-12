use crate::{
    consts::{LATEST_TAILWIND_VERSION, TAILWIND_BIN_DIR, TAILWIND_CLI_PATH},
    validate,
};

use eyre::{eyre, Context, Result};
use log::{info, warn};

pub fn check_or_install() -> Result<()> {
    // if node is available return, we can just use npx
    if validate::check_node_deps().is_ok() {
        info!("Node is installed, will use tailwind from npm");
        return Ok(());
    };

    // if tailwind standalone cli is avilable use it
    if TAILWIND_CLI_PATH.exists() {
        info!("Tailwind standalone CLI is available, will use it");
        return Ok(());
    }

    // Node and tailwind cli not present, must download
    warn!("Node is not installed, installing standalone cli");

    clean_and_download_cli()?;
    info!("Successfully downloaded tailwind cli");

    // make cli executable on unix, funciton is no-op on windows
    make_tailwind_cli_executable()?;

    Ok(())
}

pub fn check_npx_tailwind_works() -> Result<()> {
    // npx didn't work, lets try downloading it
    if validate::check_node_deps().is_ok()
        && crate::tailwind::cli::npx_works().is_err()
        && !TAILWIND_CLI_PATH.exists()
    {
        clean_and_download_cli()?;
    }

    Ok(())
}

pub fn clean_and_download_cli() -> Result<()> {
    // clean old version of tailwind cli if it exists
    if TAILWIND_BIN_DIR.exists() {
        info!("Cleaning up old version of tailwind cli");

        if let Err(error) = std::fs::remove_dir_all(TAILWIND_BIN_DIR.as_path()) {
            warn!("Unable to remove old version of tailwind cli: {error:?}");
        }
    }

    // download latest tailwind css cli
    info!(
        "Downloading latest tailwind css cli (v{}) from github",
        LATEST_TAILWIND_VERSION
    );

    download_tailwind_cli()
}

fn download_tailwind_cli() -> Result<()> {
    let tailwind_version = LATEST_TAILWIND_VERSION;
    let os = std::env::consts::OS;
    let arch = get_arch()?;

    let download_link = format!(
        "https://github.com/tailwindlabs/tailwindcss/releases/download/v{tailwind_version}/tailwindcss-{os}-{arch}",
    );

    let response = ureq::get(&download_link).call()?;
    let mut reader = response.into_reader();

    std::fs::create_dir_all(TAILWIND_BIN_DIR.as_path())?;

    let mut tailwind_cli_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(TAILWIND_CLI_PATH.as_path())
        .wrap_err("Unable to create tailwind bin")?;

    std::io::copy(&mut reader, &mut tailwind_cli_file)
        .wrap_err("Unable to save tailwind cli file")?;

    Ok(())
}

fn get_arch() -> Result<&'static str> {
    let arch = std::env::consts::ARCH;
    match arch {
        "x86_64" => Ok("x64"),
        "aarch64" => Ok("arm64"),
        "arm" => Ok("armv7"),
        _ => Err(eyre!("Unsupported architecture: {}", arch)),
    }
}

#[cfg(unix)]
fn make_tailwind_cli_executable() -> Result<()> {
    use std::os::unix::prelude::PermissionsExt;

    use color_eyre::Section;

    info!("Making tailwind cli executable");
    let mut perms = TAILWIND_CLI_PATH.metadata()?.permissions();
    perms.set_mode(0o755);

    std::fs::set_permissions(TAILWIND_CLI_PATH.as_path(), perms)
        .wrap_err("Unable to make Tailwind CLI executable")
        .suggestion("Please install node from nodejs and try again")
        .suggestion("Go to: https://nodejs.org/en/download")?;

    Ok(())
}

#[cfg(not(unix))]
fn make_tailwind_cli_executable() -> Result<()> {
    Ok(())
}
