use eyre::{eyre, Result};
use log::info;

use crate::{
    template::{TemplateName, TEMPLATE},
    validate,
};

pub struct InitArgs {
    pub project_name: String,
}

pub fn run(args: InitArgs) -> Result<()> {
    validate::check_node_deps()?;

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
