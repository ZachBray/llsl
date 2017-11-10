#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Protocol {
    pub name: String,
    pub description: String,
    pub version: String,
}
