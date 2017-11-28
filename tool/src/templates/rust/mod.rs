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

static LIB_TEMPLATE: Template = Template {
    name: "Rust module",
    content: include_str!("lib.hbs"),
};

static PACKAGE_TEMPLATE: Template = Template {
    name: "Rust package",
    content: include_str!("cargo_package.hbs"),
};

fn render_enums(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    for e in &renderer.root_model.enums {
        renderer.render(
            &ENUM_TEMPLATE,
            &e,
            &format!("rust/src/{}.rs", e.name.snake_case),
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
            &format!("rust/src/{}.rs", c.name.snake_case),
        )?;
    }
    Ok(())
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
struct LibModel<'a> {
    modules: BTreeSet<&'a Identifier>,
}

impl<'a> LibModel<'a> {
    fn new(protocol: &'a Protocol) -> Self {
        LibModel {
            modules: protocol
                .enums
                .iter()
                .map(|e| &e.name)
                .chain(protocol.codecs.iter().map(|c| &c.name))
                .collect::<BTreeSet<_>>(),
        }
    }
}

fn render_lib(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    let model = LibModel::new(renderer.root_model);
    renderer.render(&LIB_TEMPLATE, &model, "rust/src/lib.rs")
}

fn render_package(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    renderer.render(&PACKAGE_TEMPLATE, &renderer.root_model, "rust/Cargo.toml")
}

pub fn render_all(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    render_enums(renderer)?;
    render_codecs(renderer)?;
    render_lib(renderer)?;
    render_package(renderer)?;
    Ok(())
}
