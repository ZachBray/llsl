use std::collections::BTreeSet;
use try::*;
use model::*;
use output::Template;
use super::TemplateSink;

fn visit_enum_declarations(sink: TemplateSink) -> Try<()> {
    sink(Template {
        name: "JS enum type declarations",
        content: include_str!("enum_types.hbs"),
        render_targets: Box::new(|protocol, renderer| {
            for e in &protocol.enums {
                let file_name = format!("javascript/{}.d.ts", e.name.camel_case);
                renderer.render(&file_name, e)?;
            }
            Ok(())
        }),
    })
}

fn visit_enums(sink: TemplateSink) -> Try<()> {
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

impl<'a> CodecModel<'a> {
    fn new(codec: &'a Codec) -> Self {
        let imports = codec
            .fields
            .iter()
            .flat_map(|f| f.type_ref.iter())
            .collect::<BTreeSet<_>>();
        CodecModel { codec, imports }
    }
}

fn visit_codec_declarations(sink: TemplateSink) -> Try<()> {
    sink(Template {
        name: "JS codec type declarations",
        content: include_str!("codec_types.hbs"),
        render_targets: Box::new(|protocol, renderer| {
            for c in &protocol.codecs {
                let file_name = format!("javascript/{}.d.ts", c.name.camel_case);
                let model = CodecModel::new(c);
                renderer.render(&file_name, &model)?;
            }
            Ok(())
        }),
    })
}

fn visit_codecs(sink: TemplateSink) -> Try<()> {
    sink(Template {
        name: "JS codec",
        content: include_str!("codec.hbs"),
        render_targets: Box::new(|protocol, renderer| {
            for c in &protocol.codecs {
                let file_name = format!("javascript/{}.js", c.name.camel_case);
                let model = CodecModel::new(c);
                renderer.render(&file_name, &model)?;
            }
            Ok(())
        }),
    })
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
struct IndexModel<'a> {
    exports: Vec<&'a Identifier>,
}

impl<'a> IndexModel<'a> {
    fn new(protocol: &'a Protocol) -> Self {
        let enums = protocol.enums.iter().map(|e| &e.name);
        let codecs = protocol.codecs.iter().map(|c| &c.name);
        let exports = enums.chain(codecs).collect::<Vec<_>>();
        IndexModel { exports }
    }
}

fn visit_index(sink: TemplateSink) -> Try<()> {
    sink(Template {
        name: "JS index",
        content: include_str!("index.hbs"),
        render_targets: Box::new(|protocol, renderer| {
            let file_name = "javascript/index.js";
            let model = IndexModel::new(protocol);
            renderer.render(&file_name, &model)
        }),
    })
}

fn visit_index_declaration(sink: TemplateSink) -> Try<()> {
    sink(Template {
        name: "JS index type declaration",
        content: include_str!("index_types.hbs"),
        render_targets: Box::new(|protocol, renderer| {
            let file_name = "javascript/index.d.ts";
            let model = IndexModel::new(protocol);
            renderer.render(&file_name, &model)
        }),
    })
}

pub fn visit_all(sink: TemplateSink) -> Try<()> {
    visit_enums(sink)?;
    visit_enum_declarations(sink)?;
    visit_codecs(sink)?;
    visit_codec_declarations(sink)?;
    visit_index(sink)?;
    visit_index_declaration(sink)
}
