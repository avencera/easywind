use color_eyre::Help;
use eyre::{Context, Result};

pub fn check_node_deps() -> Result<()> {
    check_exists("node")?;
    check_exists("npm")?;
    check_exists("npx")?;

    Ok(())
}

pub fn check_exists(name: &str) -> Result<()> {
    which::which(name)
        .wrap_err(format!("{} not found in PATH", name))
        .suggestion(suggest_help(name))?;

    Ok(())
}

fn suggest_help(name: &str) -> String {
    let node_bins = ["node", "npm", "npx"];

    if node_bins.contains(&name) {
        "Please install node from nodejs and try again. Go to: https://nodejs.org/en/download"
            .to_string()
    } else {
        format!("Please install {} and try again", name)
    }
}
