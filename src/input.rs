#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum Endianness {
    #[serde(rename = "big")]
    Big,
    #[serde(rename = "little")]
    Little,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct EnumCaseDefinition {
    pub name: String,
    pub value: u32,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct EnumDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub cases: Vec<EnumCaseDefinition>,
    pub bits: u32,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(tag = "kind")]
pub enum TypeReference {
    #[serde(rename = "unsigned")]
    Unsigned {
        #[serde(default)]
        align: u32,
        bits: u32,
        #[serde(default)]
        padding: u32,
    },
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "custom")]
    Custom {
        #[serde(rename = "ref")]
        name: String,
    },
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct FieldDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "type")]
    pub type_ref: TypeReference,
    #[serde(default)]
    pub new_line: bool,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CodecDefinition {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Document {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub version: String,
    pub endianness: Endianness,
    pub enums: Vec<EnumDefinition>,
    pub codecs: Vec<CodecDefinition>,
}
