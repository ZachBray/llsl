use std::u32;
use std::collections::BTreeMap;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum Endianness {
    #[serde(rename = "big")]
    Big,
    #[serde(rename = "little")]
    Little,
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct EnumCaseDefinition {
    pub name: String,
    pub value: u32,
    #[serde(default)]
    pub description: String,
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct EnumDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub cases: Vec<EnumCaseDefinition>,
    pub bits: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypeReference {
    Bool,
    U8,
    U16,
    U32,
    Blob,
    Custom { name: String },
}

impl TypeReference {
    pub fn get_name(&self) -> &str {
        match self {
            &TypeReference::Bool => "bool",
            &TypeReference::U8 => "u8",
            &TypeReference::U16 => "u16",
            &TypeReference::U32 => "u32",
            &TypeReference::Blob => "Blob",
            &TypeReference::Custom { ref name } => name,
        }
    }

    fn from_token(token: &str) -> Self {
        match token {
            "bool" => TypeReference::Bool,
            "byte" => TypeReference::U8,
            "u8" => TypeReference::U8,
            "u16" => TypeReference::U16,
            "u32" => TypeReference::U32,
            "blob" => TypeReference::Blob,
            _ => TypeReference::Custom { name: token.to_owned() },
        }
    }
}

impl<'de> Deserialize<'de> for TypeReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(TypeReference::from_token(&value))
    }
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct FieldDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "type")]
    pub type_ref: TypeReference,
    #[serde(default)]
    pub new_line: bool,
    #[serde(default)]
    pub padding_bits: u32,
    #[serde(default)]
    pub ignore_first_bits: u32,
    #[serde(default)]
    pub ignore_last_bits: u32,
    pub offset_bytes: u32,
}


#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Prelude {
    pub bits: u32,
    pub name: String,
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CodecDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub preludes: Vec<Prelude>,
    pub fields: Vec<FieldDefinition>,
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct OutputDefinition {
    pub docs: Option<String>,
    pub rust: Option<String>,
    pub javascript: Option<String>,
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Document {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub version: String,
    pub endianness: Endianness,
    pub output: OutputDefinition,
    #[serde(default)]
    pub enums: Vec<EnumDefinition>,
    pub codecs: Vec<CodecDefinition>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}
