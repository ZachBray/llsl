use string_morph;

#[derive(Debug, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier {
    pub original: String,
    pub camel_case: String,
    pub pascal_case: String,
    pub snake_case: String,
    pub upper_snake_case: String,
    pub kebab_case: String,
}

impl Identifier {
    pub fn new(original: &str) -> Identifier {
        Identifier {
            original: original.to_owned(),
            camel_case: string_morph::to_camel_case(original),
            pascal_case: string_morph::to_pascal_case(original),
            snake_case: string_morph::to_snake_case(original),
            upper_snake_case: string_morph::to_snake_caps_case(original),
            kebab_case: string_morph::to_kebab_case(original),
        }
    }
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Unsigned {
    original: u32,
    hex: String,
    binary: String,
}

impl Unsigned {
    pub fn new(original: u32) -> Self {
        Unsigned {
            original,
            hex: format!("{:#x}", original),
            binary: format!("{:#b}", original),
        }
    }
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct EnumCase {
    pub name: Identifier,
    pub description: String,
    pub value: Unsigned,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Enum {
    pub name: Identifier,
    pub description: String,
    pub cases: Vec<EnumCase>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct TypeInfo {
    // The intermediate type used to read/write
    pub access_type: Identifier,
    pub is_bool: bool,
    pub is_numeric: bool,
    pub is_byte: bool,
    pub is_u16: bool,
    pub is_u32: bool,
    pub is_enum: bool,
    pub is_codec: bool,
    pub is_blob: bool,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct MemoryLocation {
    pub offset_bytes: u32,
    pub minimum_size_bytes: u32,
    pub bit_mask: Unsigned,
    pub shift: u32,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct DiagramInfo {
    pub alias: String,
    pub alias_remainder: String,
    pub starts_new_line: bool,
    pub padding_bits: u32,
    pub offset_bits: u32,
    pub padded_offset_bits: u32,
    pub bits: Option<u32>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Field {
    pub name: Identifier,
    pub type_ref: Option<Identifier>,
    pub type_info: TypeInfo,
    pub diagram_info: DiagramInfo,
    pub description: String,
    pub location: MemoryLocation,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Codec {
    pub name: Identifier,
    pub description: String,
    pub diagram: String,
    pub fields: Vec<Field>,
    pub minimum_size_bytes: u32,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Output {
    pub docs: Option<String>,
    pub rust: Option<String>,
    pub javascript: Option<String>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Protocol {
    pub name: Identifier,
    pub description: String,
    pub version: String,
    pub enums: Vec<Enum>,
    pub codecs: Vec<Codec>,
    pub output: Output,
}
