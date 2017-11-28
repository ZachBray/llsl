use std::collections::BTreeSet;
use try::*;
use model::*;
use output::*;

static CODEC_TEMPLATE: Template = Template {
    name: "Rust codec",
    content: include_str!("codec.hbs"),
};

static ENUM_TEMPLATE: Template = Template {
    name: "Rust enum",
    content: include_str!("enum.hbs"),
};

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
struct CodecModel<'a> {
    codec: &'a Codec,
    imports: BTreeSet<&'a Identifier>,
}

fn render_enums(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    for e in &renderer.root_model.enums {
        renderer.render(
            &ENUM_TEMPLATE,
            &e,
            &format!("rust/{}.rs", e.name.snake_case),
        )?;
    }
    Ok(())
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

fn render_codecs(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    for c in &renderer.root_model.codecs {
        let model = CodecModel::new(c);
        renderer.render(
            &CODEC_TEMPLATE,
            &model,
            &format!("rust/{}.rs", c.name.snake_case),
        )?;
    }
    Ok(())
}

pub fn render_all(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    render_enums(renderer)?;
    render_codecs(renderer)?;
    Ok(())
}
