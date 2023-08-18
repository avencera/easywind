use color_eyre::Help;
use eyre::{eyre, Context, Result};
use log::{info, warn};

use crate::{
    consts::{LATEST_TAILWIND_VERSION, TAILWIND_BIN_DIR, TAILWIND_CLI_PATH},
    template::{TemplateName, TEMPLATE},
    validate,
};

pub struct InitArgs {
    pub project_name: String,
}

pub fn run(args: InitArgs) -> Result<()> {
    check_or_install_tailwind()?;

    // create dirs
    create_project_dir(&args.project_name)?;
    std::fs::create_dir_all(format!("{}/src", args.project_name))?;
    std::fs::create_dir_all(format!("{}/dist", args.project_name))?;

    // create files
    // tailwind.config.js
    let ctx: minijinja::Value = minijinja::context! {};
    let template = TEMPLATE.render(TemplateName::TailwindConfig, &ctx);
    let file_path = format!("{}/tailwind.config.js", args.project_name);
    std::fs::write(file_path, template)?;

    // index.html
    let ctx: minijinja::Value = minijinja::context! { project_name => args.project_name.clone() };
    let template = TEMPLATE.render(TemplateName::ProjectIndex, &ctx);
    let file_path = format!("{}/index.html", args.project_name);
    std::fs::write(file_path, template)?;

    // src/app.css
    let ctx: minijinja::Value = minijinja::context! { project_name => args.project_name.clone() };
    let template = TEMPLATE.render(TemplateName::ProjectCss, &ctx);
    let file_path = format!("{}/src/app.css", args.project_name);
    std::fs::write(file_path, template)?;

    // dist/app.css
    let file_path = format!("{}/dist/app.css", args.project_name);
    std::fs::write(file_path, "")?;

    info!("Created project {}", args.project_name);
    info!("Run `easywind start {}` to get to work", args.project_name);

    Ok(())
}

fn create_project_dir(project_name: &str) -> Result<()> {
    let dir = std::path::Path::new(project_name);

    if dir.exists() {
        return Err(eyre!(
            "directory {project_name}/ already exists, please try a different project name",
        ));
    }

    std::fs::create_dir_all(dir)?;

    Ok(())
}

fn check_or_install_tailwind() -> Result<()> {
    // if node is available return, we can just use npx
    if validate::check_node_deps().is_ok() {
        info!("Node is installed, will use tailwind from npm");
        return Ok(());
    };

    // if node is not available, try to install it
    if TAILWIND_CLI_PATH.exists() {
        info!("Tailwind standalone CLI is available, will use it");
        return Ok(());
    }

    warn!("Node is not installed, installing standalone cli");

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

    download_tailwind_cli()?;
    info!("Successfully downloaded tailwind cli");

    // make cli executable on unix
    #[cfg(unix)]
    make_tailwind_cli_executable()?;

    Ok(())
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

fn make_tailwind_cli_executable() -> Result<()> {
    use std::os::unix::prelude::PermissionsExt;

    info!("Making tailwind cli executable");
    let mut perms = TAILWIND_CLI_PATH.metadata()?.permissions();
    perms.set_mode(0o755);

    std::fs::set_permissions(TAILWIND_CLI_PATH.as_path(), perms)
        .wrap_err("Unable to make Tailwind CLI executable")
        .suggestion("Please install node from nodejs and try again")
        .suggestion("Go to: https://nodejs.org/en/download")?;

    Ok(())
}
