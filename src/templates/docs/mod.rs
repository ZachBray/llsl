use try::*;
use output::Template;
use super::TemplateSink;

pub fn visit_all(sink: TemplateSink) -> Try<()> {
    sink(Template {
        name: "readme",
        content: include_str!("readme.hbs"),
        render_targets: Box::new(|protocol, renderer| renderer.render("README.md", protocol)),
    })
}
