use try::*;
use clap::{Arg, App};

#[derive(Debug)]
pub struct Args {
    pub input: String,
    pub output_directory: String,
}

impl Args {
    pub fn parse() -> Try<Self> {
        let matches = App::new("llslc")
            .about("Low-level serialization language (LLSL) compiler")
            .args(
                &[
                    Arg::from_usage("-i, --in=[FILE] 'The input file'").required(true),
                    Arg::from_usage("-o, --out=[FILE] 'The output directory'").required(true),
                ],
            )
            .get_matches();
        let input = matches
            .value_of("in")
            .ok_or(ErrorCode::IncorrectArguments)?
            .to_owned();
        let output_directory = matches
            .value_of("out")
            .ok_or(ErrorCode::IncorrectArguments)?
            .to_owned();
        Ok(Args {
            input,
            output_directory,
        })

    }
}
