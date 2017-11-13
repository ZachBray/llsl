use std::collections::HashMap;
use super::try::*;
use super::input::*;
use super::model::*;
use super::diagram::*;

struct TransformContext<'a> {
    document: &'a Document,
    codecs_by_name: HashMap<&'a str, &'a CodecDefinition>,
    enums_by_name: HashMap<&'a str, &'a EnumDefinition>,
    bits_by_name: HashMap<&'a str, u32>,
}

fn try_collect<In, Out, F>(mut iter: In, mut f: F) -> Try<Vec<Out>>
where
    In: Iterator,
    F: FnMut(In::Item) -> Try<Out>,
{
    let mut ys = vec![];
    while let Some(x) = iter.next() {
        ys.push(f(x)?);
    }
    Ok(ys)
}

impl<'a> TransformContext<'a> {
    fn transform_enum_case(def: &EnumCaseDefinition) -> EnumCase {
        EnumCase {
            name: Identifier::new(&def.name),
            description: def.description.to_owned(),
            value: Unsigned::new(def.value),
        }
    }

    fn transform_enum(def: &EnumDefinition) -> Enum {
        Enum {
            name: Identifier::new(&def.name),
            description: def.description.to_owned(),
            cases: def.cases
                .iter()
                .map(TransformContext::transform_enum_case)
                .collect::<Vec<_>>(),
        }
    }

    fn field_bits(&mut self, def: &'a FieldDefinition) -> Try<u32> {
        match &def.type_ref {
            &TypeReference::Bool => Ok(1),
            &TypeReference::Unsigned { bits, .. } => Ok(bits),
            &TypeReference::Custom { ref name } => self.type_bits(&name),
        }
    }

    fn codec_bits(&mut self, def: &'a CodecDefinition) -> Try<u32> {
        def.fields.iter().map(|f| self.field_bits(f)).sum()
    }

    fn type_bits(&mut self, name: &'a str) -> Try<u32> {
        if let Some(bits) = self.bits_by_name.get(name).cloned() {
            Ok(bits)
        } else {
            // N.B., we temporarily remove mappings here to prevent recursive references.
            let bits = if let Some(codec) = self.codecs_by_name.remove(name) {
                let bits = self.codec_bits(codec)?;
                self.codecs_by_name.insert(name, codec);
                Ok(bits)
            } else if let Some(e) = self.enums_by_name.get(name) {
                Ok(e.bits)
            } else {
                Err(ErrorCode::FailedToLocateType(name.to_owned()))
            }?;
            self.bits_by_name.insert(name, bits);
            Ok(bits)
        }

    }

    fn type_padding(type_ref: &TypeReference) -> u32 {
        match type_ref {
            &TypeReference::Unsigned { padding, .. } => padding,
            &TypeReference::Bool => 0,
            &TypeReference::Custom { .. } => 0,
        }
    }

    fn build_diagram(&mut self, fields: &'a Vec<FieldDefinition>) -> Try<String> {
        let mut diagram = Diagram::new();
        for def in fields {
            if def.new_line {
                diagram.align_word();
            }
            let padding = TransformContext::type_padding(&def.type_ref);
            diagram.pad(padding);
            let bits = self.field_bits(def)?;
            diagram.append(def.name.to_owned(), bits - padding);
        }
        Ok(diagram.draw())
    }

    fn transform_codec(&mut self, def: &'a CodecDefinition) -> Try<Codec> {
        Ok(Codec {
            name: Identifier::new(&def.name),
            description: def.description.to_owned(),
            diagram: self.build_diagram(&def.fields)?,
        })
    }

    fn build_model(&mut self) -> Try<Protocol> {
        Ok(Protocol {
            name: self.document.name.to_owned(),
            description: self.document.description.to_owned(),
            version: self.document.version.to_owned(),
            enums: self.document
                .enums
                .iter()
                .map(TransformContext::transform_enum)
                .collect::<Vec<_>>(),
            codecs: try_collect(self.document.codecs.iter(), |c| self.transform_codec(c))?,
        })
    }

    fn new(document: &'a Document) -> Self {
        TransformContext {
            document,
            codecs_by_name: document.codecs.iter().fold(
                HashMap::new(),
                |mut acc, ref c| {
                    acc.insert(&c.name, c);
                    acc
                },
            ),
            enums_by_name: document.enums.iter().fold(
                HashMap::new(),
                |mut acc, ref e| {
                    acc.insert(&e.name, e);
                    acc
                },
            ),
            bits_by_name: HashMap::new(),
        }
    }
}

pub fn transform(document: Document) -> Try<Protocol> {
    let mut context = TransformContext::new(&document);
    context.build_model()
}
