use std::collections::HashMap;
use super::try::*;
use super::input::*;
use super::model::*;
use super::diagram::*;

struct TransformContext<'a> {
    document: &'a Document,
    codecs_by_name: HashMap<&'a str, &'a CodecDefinition>,
    enums_by_name: HashMap<&'a str, &'a EnumDefinition>,
    bits_by_name: HashMap<&'a str, Option<u32>>,
}

fn try_fold<In, Out, F>(mut iter: In, seed: Out, mut f: F) -> Try<Out>
where
    In: Iterator,
    F: FnMut(Out, In::Item) -> Try<Out>,
{
    let mut acc = seed;
    while let Some(x) = iter.next() {
        acc = f(acc, x)?;
    }
    Ok(acc)
}


fn try_collect<In, Out, F>(iter: In, mut f: F) -> Try<Vec<Out>>
where
    In: Iterator,
    F: FnMut(In::Item) -> Try<Out>,
{
    try_fold(iter, vec![], |mut acc, x| {
        acc.push(f(x)?);
        Ok(acc)
    })
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

    fn field_bits(&mut self, def: &'a FieldDefinition) -> Try<Option<u32>> {
        match &def.type_ref {
            &TypeReference::Bool => Ok(Some(1)),
            &TypeReference::Unsigned { bits, .. } => Ok(Some(bits)),
            &TypeReference::Blob { .. } => Ok(None),
            &TypeReference::Custom { ref name } => self.type_bits(&name),
        }
    }

    fn codec_bits(&mut self, def: &'a CodecDefinition) -> Try<Option<u32>> {
        try_fold(def.fields.iter(), Some(0), |maybe_acc, f| {
            let maybe_bits = self.field_bits(f)?;
            Ok(maybe_acc.and_then(|acc| maybe_bits.map(|bits| bits + acc)))
        })
    }

    fn type_bits(&mut self, name: &'a str) -> Try<Option<u32>> {
        if let Some(bits) = self.bits_by_name.get(name).cloned() {
            Ok(bits)
        } else {
            // N.B., we temporarily remove mappings here to prevent recursive references.
            let bits = if let Some(codec) = self.codecs_by_name.remove(name) {
                let bits = self.codec_bits(codec)?;
                self.codecs_by_name.insert(name, codec);
                Ok(bits)
            } else if let Some(e) = self.enums_by_name.get(name) {
                Ok(Some(e.bits))
            } else {
                Err(ErrorCode::FailedToLocateType(name.to_owned()))
            }?;
            self.bits_by_name.insert(name, bits);
            Ok(bits)
        }

    }

    fn transform_type_info(&mut self, type_ref: &'a TypeReference) -> TypeInfo {
        let mut info = TypeInfo {
            is_bool: false,
            is_enum: false,
            is_codec: false,
            is_unsigned: false,
            is_blob: false,
        };
        match type_ref {
            &TypeReference::Bool => info.is_bool = true,
            &TypeReference::Unsigned { .. } => info.is_unsigned = true,
            &TypeReference::Custom { ref name } => {
                let key: &str = name;
                info.is_enum = self.enums_by_name.contains_key(&key);
                info.is_codec = self.codecs_by_name.contains_key(&key);
            }
            &TypeReference::Blob { .. } => info.is_blob = true,
        };
        info
    }

    fn transform_field(
        &mut self,
        diagram: &mut Diagram,
        mut offset: u32,
        def: &'a FieldDefinition,
    ) -> Try<Field> {
        offset += def.skip; // Skip before calculating alignment
        let alignment_padding = if def.alignment > 0 {
            (def.alignment - (offset % def.alignment)) % def.alignment
        } else {
            0
        };
        diagram.pad('/', def.skip + alignment_padding);
        offset += alignment_padding; // Add alignment padding to offset
        if def.new_line {
            diagram.align_word();
        }
        diagram.pad('0', def.padding);
        offset += def.padding; // Add padding to offset
        let bits = self.field_bits(def)?;
        let title_size = match bits {
            Some(bits) => diagram.append(def.name.to_owned(), bits),
            None => diagram.append_unsized(def.name.to_owned()),
        };
        let diagram_alias = def.name[..title_size].to_owned();
        let diagram_alias_remainder = def.name[title_size..].to_owned();
        Ok(Field {
            name: Identifier::new(&def.name),
            type_ref: def.type_ref.get_custom_name().map(|n| Identifier::new(n)),
            type_info: self.transform_type_info(&def.type_ref),
            description: def.description.to_owned(),
            diagram_alias,
            diagram_alias_remainder,
            offset,
            bits,
        })
    }

    fn transform_codec(&mut self, def: &'a CodecDefinition) -> Try<Codec> {
        let mut diagram = Diagram::new();
        let mut offset = 0;
        let mut fields = vec![];
        for prelude in &def.preludes {
            diagram.append(prelude.name.to_owned(), prelude.bits);
            offset += prelude.bits;
        }
        for def in &def.fields {
            let field = self.transform_field(&mut diagram, offset, def)?;
            offset = field.offset + field.bits.unwrap_or(0);
            fields.push(field);
        }
        Ok(Codec {
            name: Identifier::new(&def.name),
            description: def.description.to_owned(),
            diagram: diagram.draw(),
            fields,
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
