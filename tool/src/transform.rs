use std::collections::HashMap;
use std::cmp::{min, max};
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

fn create_bit_mask(bits: u32, ignore_first_bits: u32, ignore_last_bits: u32) -> Unsigned {
    let mut mask = 0;
    let mask_size = max(bits, ignore_first_bits + ignore_last_bits + 1); // For booleans with ignore_first...
    for i in 0..mask_size {
        let is_set = i >= ignore_first_bits && mask_size - i > ignore_last_bits;
        if is_set {
            mask = mask | (1 << i);
        }
    }
    Unsigned::new(mask)
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
            &TypeReference::Byte => Ok(Some(8)),
            &TypeReference::U16 => Ok(Some(16)),
            &TypeReference::U32 => Ok(Some(32)),
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
            access_type: None,
            is_bool: false,
            is_numeric: false,
            is_byte: false,
            is_u16: false,
            is_u32: false,
            is_enum: false,
            is_codec: false,
            is_blob: false,
        };
        match type_ref {
            &TypeReference::Bool => {
                info.access_type = Some(Identifier::new("bool"));
                info.is_bool = true;
            }
            &TypeReference::Byte => {
                info.access_type = Some(Identifier::new("byte"));
                info.is_numeric = true;
                info.is_byte = true;
            }
            &TypeReference::U16 => {
                info.access_type = Some(Identifier::new("u16"));
                info.is_numeric = true;
                info.is_u16 = true;
            }
            &TypeReference::U32 => {
                info.access_type = Some(Identifier::new("u32"));
                info.is_numeric = true;
                info.is_u32 = true;
            }
            &TypeReference::Custom { ref name } => {
                let key: &str = name;
                info.is_enum = self.enums_by_name.contains_key(&key);
                if info.is_enum {
                    info.access_type = Some(Identifier::new("u32"));
                }
                info.is_codec = self.codecs_by_name.contains_key(&key);
            }
            &TypeReference::Blob { .. } => {
                info.is_blob = true;
            }
        };
        info
    }

    fn transform_field(&mut self, def: &'a FieldDefinition) -> Try<Field> {
        let bits = self.field_bits(def)?;
        let offset_bits = def.offset_bytes * 8 + def.ignore_first_bits;
        trace!("Creating field {:?}", def);
        Ok(Field {
            name: Identifier::new(&def.name),
            description: def.description.to_owned(),
            type_ref: def.type_ref.get_custom_name().map(|n| Identifier::new(n)),
            type_info: self.transform_type_info(&def.type_ref),
            diagram_info: DiagramInfo {
                alias: "".to_owned(),
                alias_remainder: "".to_owned(),
                offset_bits,
                padding_bits: def.padding_bits,
                padded_offset_bits: def.padding_bits + offset_bits,
                starts_new_line: def.new_line,
                bits: bits.map(|b| {
                    // Booleans may use ignore_first_bits to position flag inside a byte
                    b -
                        min(
                            b - 1,
                            def.padding_bits + def.ignore_first_bits + def.ignore_last_bits,
                        )
                }),
            },
            location: MemoryLocation {
                offset_bytes: def.offset_bytes,
                bit_mask: bits.map(|b| {
                    create_bit_mask(
                        b,
                        def.ignore_first_bits + def.padding_bits,
                        def.ignore_last_bits,
                    )
                }).unwrap_or(Unsigned::new(0)),
                shift: def.ignore_first_bits,
            },
        })
    }

    fn create_diagram(
        &mut self,
        def: &'a CodecDefinition,
        fields: &mut Vec<Field>,
    ) -> Try<Diagram> {
        trace!("Creating diagram for {}", def.name);
        let mut diagram = Diagram::new();
        let mut next_free_offset = 0;
        for prelude in &def.preludes {
            diagram.append(prelude.name.to_owned(), prelude.bits);
            next_free_offset += prelude.bits;
        }
        for field in &mut fields.iter_mut() {
            trace!(
                "Attempting to add section for {} (field offset: {}, next free offset: {})",
                field.name.original,
                field.diagram_info.offset_bits,
                next_free_offset
            );
            let overlaps_with_previous_field = field.diagram_info.offset_bits < next_free_offset;
            if overlaps_with_previous_field {
                Err(ErrorCode::FailedToLayoutDiagramDueToOverlappingFields(
                    field.name.original.to_owned(),
                ))?;
            }
            let skip_bits = field.diagram_info.offset_bits - next_free_offset;
            diagram.pad('/', skip_bits);
            next_free_offset += skip_bits;
            if field.diagram_info.starts_new_line {
                diagram.align_word();
            }
            diagram.pad('0', field.diagram_info.padding_bits);
            next_free_offset += field.diagram_info.padding_bits;
            let title_size = match field.diagram_info.bits {
                Some(bits) => {
                    next_free_offset += bits;
                    diagram.append(field.name.original.to_owned(), bits)
                }
                None => diagram.append_unsized(field.name.original.to_owned()),
            };
            field.diagram_info.alias = field.name.original[..title_size].to_owned();
            field.diagram_info.alias_remainder = field.name.original[title_size..].to_owned();
        }
        Ok(diagram)
    }

    fn transform_codec(&mut self, def: &'a CodecDefinition) -> Try<Codec> {
        let mut fields = try_collect(def.fields.iter(), |f| self.transform_field(f))?;
        fields.sort_by_key(|f| f.diagram_info.offset_bits);

        let diagram = self.create_diagram(def, &mut fields)?;

        Ok(Codec {
            name: Identifier::new(&def.name),
            description: def.description.to_owned(),
            diagram: diagram.draw(),
            fields,
        })
    }

    fn build_model(&mut self) -> Try<Protocol> {
        Ok(Protocol {
            name: Identifier::new(&self.document.name),
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
