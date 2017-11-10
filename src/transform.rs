use super::input::*;
use super::model::*;

fn transform_enum_case(def: EnumCaseDefinition) -> EnumCase {
    EnumCase {
        name: Identifier::new(&def.name),
        description: def.description,
        value: Unsigned::new(def.value),
    }
}

fn transform_enum(def: EnumDefinition) -> Enum {
    Enum {
        name: Identifier::new(&def.name),
        description: def.description,
        cases: def.cases
            .into_iter()
            .map(transform_enum_case)
            .collect::<Vec<_>>(),
    }
}

fn transform_codec(def: CodecDefinition) -> Codec {
    Codec {
        name: Identifier::new(&def.name),
        description: def.description,
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
        codecs: document
            .codecs
            .into_iter()
            .map(transform_codec)
            .collect::<Vec<_>>(),
    }
}
