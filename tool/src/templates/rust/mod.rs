use std::collections::BTreeSet;
use try::*;
use model::*;
use output::*;

static CODEC_TEMPLATE: Template = Template {
    name: "Rust codec",
    content: include_str!("codec.hbs"),
};

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
    render_codecs(renderer)?;
    Ok(())
}
