use std::u32;
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
pub enum Offset {
    Fixed { bits: u32 },
    Evaluated { field_name: String },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypeReference {
    Bool,
    Unsigned { bits: u32 },
    Blob { offset: Offset },
    Custom { name: String },
}

impl TypeReference {
    pub fn get_custom_name(&self) -> Option<&str> {
        match self {
            &TypeReference::Bool => None,
            &TypeReference::Unsigned { .. } => None,
            &TypeReference::Blob { .. } => None,
            &TypeReference::Custom { ref name } => Some(name),
        }
    }

    fn from_token(token: &str) -> Self {
        if token == "bool" {
            TypeReference::Bool
        } else if let Some(bits) = TypeReference::try_unsigned(token) {
            TypeReference::Unsigned { bits }
        } else if let Some(offset) = TypeReference::try_blob(token) {
            TypeReference::Blob { offset }
        } else {
            TypeReference::Custom { name: token.to_owned() }
        }
    }

    fn try_unsigned(token: &str) -> Option<u32> {
        if token.len() >= 2 && &token[token.len() - 1..] == "u" {
            token[..token.len() - 1].parse::<u32>().ok()
        } else {
            None
        }
    }

    fn try_blob(token: &str) -> Option<Offset> {
        if token.starts_with("@") {
            let address = &token[1..];
            address
                .parse::<u32>()
                .ok()
                .map(|bits| Offset::Fixed { bits })
                .or(Some(Offset::Evaluated { field_name: address.to_owned() }))
        } else {
            None
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
    pub padding: u32,
    #[serde(default)]
    pub skip: u32,
    #[serde(default)]
    pub alignment: u32,
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CodecDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub fields: Vec<FieldDefinition>,
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Document {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub version: String,
    pub endianness: Endianness,
    #[serde(default)]
    pub enums: Vec<EnumDefinition>,
    pub codecs: Vec<CodecDefinition>,
}
