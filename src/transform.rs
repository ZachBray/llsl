use super::input::*;
use super::model::*;

fn transform_enum(decl: EnumDefinition) -> Enum {
    Enum {
        name: Identifier::new(&decl.name),
        description: decl.description,
    }
}

pub fn transform(document: Document) -> Protocol {
    Protocol {
        name: document.name,
        description: document.description,
        version: document.version,
        enums: document
            .enums
            .into_iter()
            .map(transform_enum)
            .collect::<Vec<_>>(),
    }
}
