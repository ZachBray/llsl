use std::io;
use serde_yaml;
use handlebars;

quick_error! {
    #[derive(Debug)]
    pub enum ToolError {
        IncorrectArguments {
            description("Incorrect arguments supplied.")
        }
        FailedToPerformIO(err: io::Error) {
            description("Failed to perform I/O")
            display("Failed to perform I/O: {}", err)
            cause(err)
            from()
        }
        FailedToParseInputFile(err: serde_yaml::Error) {
            description("Failed to parse input file")
            display("Failed to parse input file: {}", err)
            cause(err)
            from()
        }
        FailedToExecuteTemplate(err: handlebars::TemplateRenderError) {
            description("Failed to execute code generation template.")
            display("Failed to execute code generation template: {}", err)
            cause(err)
            from()
        }
        FailedToLocateType(type_name: String) {
            description("Failed to locate type")
            display("Failed to locate type: {}", type_name)
        }
        FailedToLayoutDiagramDueToOverlappingFields(field: String) {
            description("Failed to layout diagram. Some fields were overlapping.")
            display("Failed to layout diagram. This field has some overlap with another: {}", field)
        }
    }
}

pub type Try<T> = Result<T, ToolError>;
