use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub enum Endianness {
    #[serde(rename = "big")]
    Big,
    #[serde(rename = "little")]
    Little,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct EnumCaseDefinition {
    pub name: String,
    pub value: u32,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct EnumDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub cases: Vec<EnumCaseDefinition>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypeReference {
    Unsigned,
    Bool,
    Custom { name: String },
}

impl TypeReference {
    fn from_token(token: &str) -> Self {
        match token {
            "unsigned" => TypeReference::Unsigned,
            "bool" => TypeReference::Bool,
            custom => TypeReference::Custom { name: custom.to_owned() },
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

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct FieldDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "type")]
    pub type_ref: TypeReference,
    #[serde(default)]
    pub new_line: bool,
    #[serde(default)]
    pub align: u32,
    pub bits: Option<u32>,
    #[serde(default)]
    pub padding: u32,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct CodecDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Document {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub version: String,
    pub endianness: Endianness,
    pub enums: Vec<EnumDefinition>,
    pub codecs: Vec<CodecDefinition>,
}
