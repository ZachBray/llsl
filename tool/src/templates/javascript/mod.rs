use std::collections::BTreeSet;
use try::*;
use model::*;
use output::Template;
use super::TemplateSink;

pub fn visit_enums(sink: TemplateSink) -> Try<()> {
    sink(Template {
        name: "JS enum",
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

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
struct CodecModel<'a> {
    codec: &'a Codec,
    imports: BTreeSet<&'a Identifier>,
}

pub fn visit_codecs(sink: TemplateSink) -> Try<()> {
    sink(Template {
        name: "JS codec",
        content: include_str!("codec.hbs"),
        render_targets: Box::new(|protocol, renderer| {
            for c in &protocol.codecs {
                let file_name = format!("javascript/{}.js", c.name.camel_case);
                let imports = c.fields
                    .iter()
                    .flat_map(|f| f.type_ref.iter())
                    .collect::<BTreeSet<_>>();
                let model = CodecModel { codec: &c, imports };
                renderer.render(&file_name, &model)?;
            }
            Ok(())
        }),
    })
}

pub fn visit_all(sink: TemplateSink) -> Try<()> {
    visit_enums(sink)?;
    visit_codecs(sink)
}
