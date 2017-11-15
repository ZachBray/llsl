use string_morph;

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
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
}

impl Unsigned {
    pub fn new(original: u32) -> Self {
        Unsigned {
            original,
            hex: format!("{:#X}", original),
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
pub struct Field {
    pub name: Identifier,
    pub diagram_alias: String,
    pub diagram_alias_remainder: String,
    pub description: String,
    pub offset: u32,
    pub bits: u32,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Codec {
    pub name: Identifier,
    pub description: String,
    pub diagram: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Protocol {
    pub name: String,
    pub description: String,
    pub version: String,
    pub enums: Vec<Enum>,
    pub codecs: Vec<Codec>,
}
