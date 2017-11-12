use std::io;
use serde_yaml;
use handlebars;

#[derive(Debug)]
pub enum ErrorCode {
    IncorrectArguments,
    FailedToOpenInputFile(io::Error),
    FailedToParseInputFile(serde_yaml::Error),
    FailedToCreateOutputDir(io::Error),
    FailedToCreateOutputFile(io::Error),
    FailedToExecuteTemplate(handlebars::TemplateRenderError),
    FailedToLocateType(String),
}

/*
impl ErrorCode {
    pub fn wrap<T>(self) -> Try<T> {
        Err(self)
    }
}
*/

pub type Try<T> = Result<T, ErrorCode>;
