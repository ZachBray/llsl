use std::collections::BTreeSet;
use try::*;
use model::*;
use output::*;

static ENUM_TEMPLATE: Template = Template {
    name: "JS enum",
    content: include_str!("enum.hbs"),
};

static ENUM_TYPES_TEMPLATE: Template = Template {
    name: "JS enum_types",
    content: include_str!("enum_types.hbs"),
};

static CODEC_TEMPLATE: Template = Template {
    name: "JS codec",
    content: include_str!("codec.hbs"),
};

static CODEC_TYPES_TEMPLATE: Template = Template {
    name: "JS codec_types",
    content: include_str!("codec_types.hbs"),
};

static INDEX_TEMPLATE: Template = Template {
    name: "JS index",
    content: include_str!("index.hbs"),
};

static INDEX_TYPES_TEMPLATE: Template = Template {
    name: "JS index_types",
    content: include_str!("index_types.hbs"),
};

static PACKAGE_TEMPLATE: Template = Template {
    name: "JS package",
    content: include_str!("package.hbs"),
};

fn render_enums(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    for e in &renderer.root_model.enums {
        renderer.render(
            &ENUM_TEMPLATE,
            e,
            &format!("javascript/{}.js", e.name.camel_case),
        )?;
        renderer.render(
            &ENUM_TYPES_TEMPLATE,
            e,
            &format!("javascript/{}.d.ts", e.name.camel_case),
        )?;
    }
    Ok(())
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

fn render_codecs(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    for c in &renderer.root_model.codecs {
        let model = CodecModel::new(c);
        renderer.render(
            &CODEC_TEMPLATE,
            &model,
            &format!("javascript/{}.js", c.name.camel_case),
        )?;
        renderer.render(
            &CODEC_TYPES_TEMPLATE,
            &model,
            &format!("javascript/{}.d.ts", c.name.camel_case),
        )?;
    }
    Ok(())
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

fn render_index(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    let model = IndexModel::new(renderer.root_model);
    renderer.render(
        &INDEX_TEMPLATE,
        &model,
        "javascript/index.js",
    )?;
    renderer.render(&INDEX_TYPES_TEMPLATE, &model, "javascript/index.d.ts")
}

fn render_package(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    renderer.render(
        &PACKAGE_TEMPLATE,
        renderer.root_model,
        "javascript/package.json",
    )
}

pub fn render_all(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    render_enums(renderer)?;
    render_codecs(renderer)?;
    render_index(renderer)?;
    render_package(renderer)
}
