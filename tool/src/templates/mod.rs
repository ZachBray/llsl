mod docs;
mod javascript;
mod rust;

use try::*;
use model::*;
use output::*;

pub fn render_all(renderer: &TemplateRenderer<&Protocol>) -> Try<()> {
    docs::render_all(renderer)?;
    javascript::render_all(renderer)?;
    rust::render_all(renderer)?;
    Ok(())
}
