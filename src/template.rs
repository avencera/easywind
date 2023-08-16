use minijinja::Environment;
use once_cell::sync::Lazy;

pub enum TemplateName {
    Index,
}

impl From<TemplateName> for &'static str {
    fn from(template: TemplateName) -> Self {
        match template {
            TemplateName::Index => "index",
        }
    }
}

#[cfg(not(feature = "dev"))]
pub struct Template<'a> {
    env: Environment<'a>,
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
static INDEX_FILE: &str = std::include_str!("../templates/index.html.j2");

#[cfg(not(feature = "dev"))]
pub static TEMPLATE: Lazy<Template> =
    Lazy::new(|| Template::new().add_new(TemplateName::Index, INDEX_FILE));

// DEV

#[cfg(feature = "dev")]
pub struct Template<'a> {
    env: std::sync::Mutex<Environment<'a>>,
}

impl Default for Template<'_> {
    fn default() -> Self {
        Self::new()
    }
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
    Template::new().set_loader(move |name| match name {
        "index" => {
            log::info!("loading index");
            Ok(std::fs::read_to_string("templates/index.html.j2").ok())
        }
        other => panic!("only index is allowed, not {}", other),
    })
});
