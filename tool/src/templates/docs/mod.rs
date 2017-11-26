use try::*;
use model::*;
use output::*;

static README_TEMPLATE: Template = Template {
    name: "readme",
    content: include_str!("readme.hbs"),
};

pub fn render_all(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    renderer.render(&README_TEMPLATE, renderer.root_model, "README.md")
}
