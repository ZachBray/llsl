use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use handlebars::Handlebars;
use serde::Serialize;
use super::try::*;
use super::model::*;
use super::templates;

pub struct Template<'a> {
    pub name: &'a str,
    pub content: &'a str,
}

pub struct TemplateRenderer<Model> {
    output_dir: String,
    engine: Handlebars,
    pub root_model: Model,
}

impl<Model> TemplateRenderer<Model> {
    fn new(output_dir: String, root_model: Model) -> Self {
        TemplateRenderer {
            engine: Handlebars::new(),
            output_dir,
            root_model,
        }
    }

    pub fn render<T>(&self, template: &Template, data: &T, output_path: &str) -> Try<()>
    where
        T: Serialize,
    {
        info!("Executing {} template", template.name);
        let output_path = Path::new(&self.output_dir).join(output_path);
        trace!("Template: {}", template.content);
        let mut output_file = Self::create_output_file(&output_path)?;
        self.engine.template_renderw(
            template.content,
            data,
            &mut output_file,
        )?;
        Ok(())
    }

    fn create_output_file(output_path: &PathBuf) -> Try<File> {
        Self::create_parent_dir(output_path)?;
        debug!("Creating output file: {:?}", output_path);
        let file = File::create(output_path)?;
        Ok(file)
    }

    fn create_parent_dir(output_path: &PathBuf) -> Try<()> {
        if let Some(parent_dir) = output_path.parent() {
            debug!("Creating parent directory: {:?}", parent_dir);
            create_dir_all(parent_dir)?;
        }
        Ok(())
    }
}

pub fn generate_code(protocol: &Protocol, output_dir: &str) -> Try<()> {
    let renderer = TemplateRenderer::new(output_dir.to_owned(), protocol);
    templates::render_all(&renderer)
}
