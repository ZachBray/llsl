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
            upper_snake_case: string_morph::to_snake_case(original),
            kebab_case: string_morph::to_kebab_case(original),
        }
    }
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Enum {
    pub name: Identifier,
    pub description: String,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Protocol {
    pub name: String,
    pub description: String,
    pub version: String,
    pub enums: Vec<Enum>,
}
