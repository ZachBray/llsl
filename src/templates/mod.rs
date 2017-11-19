mod docs;

use output::Template;

pub fn all() -> [Template; 1] {
    [docs::readme()]
}
