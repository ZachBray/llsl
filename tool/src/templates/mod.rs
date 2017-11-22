mod docs;
mod javascript;

use try::*;
use output::Template;

pub type TemplateSink<'a> = &'a Fn(Template) -> Try<()>;

pub fn visit_all(sink: TemplateSink) -> Try<()> {
    docs::visit_all(sink)?;
    javascript::visit_all(sink)
}
