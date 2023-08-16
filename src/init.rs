use eyre::{eyre, Result};

use crate::validate;

pub struct InitArgs {
    pub project_name: String,
}

pub fn run(args: InitArgs) -> Result<()> {
    validate::check_node_deps()?;

    create_dir(&args.project_name)?;

    Ok(())
}

fn create_dir(project_name: &str) -> Result<()> {
    let dir = std::path::Path::new(project_name);

    if dir.exists() {
        return Err(eyre!(
            "directory {project_name}/ already exists, please try a different project name",
        ));
    }

    std::fs::create_dir_all(dir)?;

    Ok(())
}

