use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use handlebars::Handlebars;
use super::try::*;
use super::model::*;

struct Template {
    name: &'static str,
    content: &'static str,
    output_file: &'static str,
}

impl Template {
    fn execute(&self, engine: &mut Handlebars, protocol: &Protocol, output_dir: &str) -> Try<()> {
        info!("Executing {} template", self.name);
        let output_path = self.resolve_output_path(output_dir);
        trace!("Template: {}", self.content);
        let mut output_file = Template::create_output_file(&output_path)?;
        engine
            .template_renderw(self.content, protocol, &mut output_file)
            .map_err(|e| ErrorCode::FailedToExecuteTemplate(e))
    }

    fn resolve_output_path(&self, output_dir: &str) -> PathBuf {
        Path::new(output_dir).join(self.output_file)
    }

    fn create_output_file(output_path: &PathBuf) -> Try<File> {
        Template::create_parent_dir(output_path)?;
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

static TEMPLATES: [Template; 1] = [
    Template {
        name: "readme",
        content: include_str!("../templates/readme.hbs"),
        output_file: "README.md",
    },
];

pub fn generate_code(protocol: &Protocol, output_dir: &str) -> Try<()> {
    let mut engine = Handlebars::new();
    for t in TEMPLATES.iter() {
        t.execute(&mut engine, protocol, output_dir)?;
    }
    Ok(())
}
