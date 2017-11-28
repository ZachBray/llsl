extern crate bit_set;
extern crate clap;
extern crate handlebars;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate quick_error;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate string_morph;

mod try;
mod args;
mod input;
mod model;
mod diagram;
mod transform;
mod output;
mod templates;

use std::fs::File;
use self::try::*;
use self::args::*;
use self::input::*;
use self::transform::transform;
use self::output::generate_code;

fn compile() -> Try<()> {
    info!("Parsing arguments");
    let args = Args::parse()?;
    debug!("Parsed arguments: {:?}", args);
    info!("Opening input file: {:?}", args.input);
    let input_file = File::open(args.input)?;
    info!("Parsing input file");
    let document: Document = serde_yaml::from_reader(input_file)?;
    debug!("Parsed input file: {:?}", document);
    info!("Building model");
    let model = transform(document)?;
    debug!("Built model: {:?}", model);
    info!("Generating code");
    generate_code(&model, &args.output_directory)
}

fn main() {
    match pretty_env_logger::init() {
        Ok(_) => {
            match compile() {
                Ok(_) => info!("Done"),
                Err(error) => error!("Failed to generate code. {}", error),
            }
        }
        Err(error) => println!("Failed to start logger. {}", error),
    }
}
