use minijinja::Environment;
use once_cell::sync::Lazy;

use strum::EnumIter;

#[cfg(not(feature = "dev"))]
use std::collections::HashMap;
#[cfg(not(feature = "dev"))]
use strum::IntoEnumIterator;

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TemplateName {
    Index,
    ProjectIndex,
    ProjectCss,
    TailwindConfig,
}

impl From<TemplateName> for &'static str {
    fn from(template: TemplateName) -> Self {
        match template {
            TemplateName::Index => "index.html",
            TemplateName::ProjectIndex => "project_index.html",
            TemplateName::TailwindConfig => "tailwind.config.js",
            TemplateName::ProjectCss => "project_app_css.css",
        }
    }
}

#[cfg(not(feature = "dev"))]
pub struct Template<'a> {
    env: Environment<'a>,
}

impl Default for Template<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "dev"))]
impl Template<'_> {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn add_new(mut self, name: TemplateName, file: &'static str) -> Self {
        self.env
            .add_template(name.into(), file)
            .expect("unable to add template");

        self
    }

    pub fn render(&self, name: TemplateName, context: &minijinja::Value) -> String {
        let template = self.env.get_template(name.into()).unwrap();
        template.render(context).expect("unable to render template")
    }
}

#[cfg(not(feature = "dev"))]
static TEMPLATES_DIR: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/templates");

#[cfg(not(feature = "dev"))]
pub static TEMPLATE_FILES: Lazy<HashMap<TemplateName, &str>> = Lazy::new(|| {
    let mut files = std::collections::HashMap::new();

    for template in TemplateName::iter() {
        let template_name: &str = template.into();
        let file_name = format!("{}.j2", template_name);

        let file = TEMPLATES_DIR
            .get_file(file_name)
            .unwrap()
            .contents_utf8()
            .expect("Unable to read template file");

        files.insert(template, file);
    }

    files
});

#[cfg(not(feature = "dev"))]
pub static TEMPLATE: Lazy<Template> = Lazy::new(|| {
    let mut template = Template::new();

    for (name, file) in TEMPLATE_FILES.iter() {
        template = template.add_new(*name, file);
    }

    template
});

// DEV

#[cfg(feature = "dev")]
pub struct Template<'a> {
    env: std::sync::Mutex<Environment<'a>>,
}

#[cfg(feature = "dev")]
impl Template<'_> {
    pub fn new() -> Self {
        Self {
            env: std::sync::Mutex::new(Environment::new()),
        }
    }

    pub fn set_loader<F>(self, func: F) -> Self
    where
        F: Fn(&str) -> Result<Option<String>, minijinja::Error> + Send + Sync + 'static,
    {
        self.env.lock().unwrap().set_loader(func);
        self
    }

    pub fn render(&self, name: TemplateName, context: &minijinja::Value) -> String {
        let mut env = self.env.lock().unwrap();
        env.clear_templates();

        env.get_template(name.into())
            .unwrap()
            .render(context)
            .expect("unable to render template")
    }
}

#[cfg(feature = "dev")]
pub static TEMPLATE: Lazy<Template> = Lazy::new(|| {
    Template::new().set_loader(move |name| {
        let file_name = format!("templates/{name}.html.j2");
        Ok(std::fs::read_to_string(&file_name).ok())
    })
});
