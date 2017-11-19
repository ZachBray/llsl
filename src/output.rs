use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use handlebars::Handlebars;
use serde::Serialize;
use super::try::*;
use super::model::*;
use super::templates;

pub struct TemplateRenderer<'a> {
    output_dir: &'a str,
    template: &'a Template,
    engine: &'a Handlebars,
}

impl<'a> TemplateRenderer<'a> {
    pub fn render<T>(&self, output_path: &str, data: &T) -> Try<()>
    where
        T: Serialize,
    {
        info!("Executing {} template", self.template.name);
        let output_path = Path::new(self.output_dir).join(output_path);
        trace!("Template: {}", self.template.content);
        let mut output_file = TemplateRenderer::create_output_file(&output_path)?;
        self.engine
            .template_renderw(self.template.content, data, &mut output_file)
            .map_err(|e| ErrorCode::FailedToExecuteTemplate(e))
    }

    fn create_output_file(output_path: &PathBuf) -> Try<File> {
        TemplateRenderer::create_parent_dir(output_path)?;
        debug!("Creating output file: {:?}", output_path);
        File::create(output_path).map_err(|e| ErrorCode::FailedToCreateOutputFile(e))
    }

    fn create_parent_dir(output_path: &PathBuf) -> Try<()> {
        if let Some(parent_dir) = output_path.parent() {
            debug!("Creating parent directory: {:?}", parent_dir);
            create_dir_all(parent_dir).map_err(|e| {
                ErrorCode::FailedToCreateOutputDir(e)
            })?;
        }
        Ok(())
    }
}

pub struct Template {
    pub name: &'static str,
    pub content: &'static str,
    pub render_targets: Box<Fn(&Protocol, &mut TemplateRenderer) -> Try<()>>,
}

pub fn generate_code(protocol: &Protocol, output_dir: &str) -> Try<()> {
    let engine = Handlebars::new();
    let render_template = |template| {
        let mut renderer = TemplateRenderer {
            output_dir,
            engine: &engine,
            template: &template,
        };
        (*template.render_targets)(protocol, &mut renderer)
    };
    templates::visit_all(&render_template)
}
