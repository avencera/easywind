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

pub struct Template<'a> {
    env: Environment<'a>,
}

impl Default for Template<'_> {
    fn default() -> Self {
        Self::new()
    }
}

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

    pub fn render(&self, name: TemplateName, context: &serde_json::Value) -> String {
        let template = self.env.get_template(name.into()).unwrap();

        template.render(context).expect("unable to render template")
    }
}

static INDEX_FILE: &str = std::include_str!("../../templates/index.html.j2");
pub static TEMPLATE: Lazy<Template> =
    Lazy::new(|| Template::new().add_new(TemplateName::Index, INDEX_FILE));
