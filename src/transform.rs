use super::input::*;
use super::model::*;

pub fn transform(document: Document) -> Protocol {
    Protocol {
        name: document.name,
        description: document.description,
        version: document.version,
    }
}
