use try::*;
use output::Template;
use super::TemplateSink;

pub fn visit_all(sink: TemplateSink) -> Try<()> {
    sink(Template {
        name: "enum",
        content: include_str!("enum.hbs"),
        render_targets: Box::new(|protocol, renderer| {
            for e in &protocol.enums {
                let file_name = format!("javascript/{}.js", e.name.camel_case);
                renderer.render(&file_name, e)?;
            }
            Ok(())
        }),
    })
}
