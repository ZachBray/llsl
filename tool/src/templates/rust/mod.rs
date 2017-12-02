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
            &format!("src/{}.rs", e.name.snake_case),
        )?;
    }
    Ok(())
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
struct CodecModel<'a> {
    codec: Codec,
    imports: BTreeSet<&'a Identifier>,
}

impl<'a> CodecModel<'a> {
    fn new(codec: &'a Codec) -> Self {
        let imports = codec
            .fields
            .iter()
            .filter(|f| f.type_info.is_enum || f.type_info.is_codec)
            .map(|f| &f.type_ref)
            .collect::<BTreeSet<_>>();
        // Add storage_type metadata to fields in order to
        // create schemas
        let fields = codec
            .fields
            .iter()
            .map(|f| {
                let mut field = f.clone();
                let storage_type =
                    if f.type_info.is_enum || f.type_info.is_codec || f.type_info.is_blob {
                        "u32".to_owned()
                    } else if f.type_info.is_bool {
                        "u8".to_owned()
                    } else {
                        f.type_ref.original.to_owned()
                    };
                field.metadata.insert(
                    "storage_type".to_owned(),
                    storage_type,
                );
                field
            })
            .collect::<Vec<_>>();
        CodecModel {
            codec: Codec {
                fields,
                ..codec.clone()
            },
            imports,
        }
    }
}

fn render_codecs(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    for c in &renderer.root_model.codecs {
        let model = CodecModel::new(c);
        renderer.render(
            &CODEC_TEMPLATE,
            &model,
            &format!("src/{}.rs", c.name.snake_case),
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
    renderer.render(&LIB_TEMPLATE, &model, "src/lib.rs")
}

fn render_package(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    renderer.render(&PACKAGE_TEMPLATE, &renderer.root_model, "Cargo.toml")
}

pub fn render_all(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    render_enums(renderer)?;
    render_codecs(renderer)?;
    render_lib(renderer)?;
    render_package(renderer)?;
    Ok(())
}
