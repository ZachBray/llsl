use try::*;
use clap::{Arg, App};

#[derive(Debug)]
pub struct Args {
    pub input: String,
}

impl Args {
    pub fn parse() -> Try<Self> {
        let matches = App::new("llslc")
            .about("Low-level serialization language (LLSL) compiler")
            .args(
                &[
                    Arg::from_usage("-i, --in=[FILE] 'The input file'").required(true),
                ],
            )
            .get_matches();
        let input = matches
            .value_of("in")
            .ok_or(ToolError::IncorrectArguments)?
            .to_owned();
        Ok(Args { input })

    }
}
