//! ```cargo
//! [dependencies]
//! structopt = "0.3"
//! log = "0.4"
//! simple_logger = "1.11"
//! anyhow = "1"
//! lazy_static = "1"
//! ```

#![warn(
clippy::all,
clippy::pedantic,
)]

use std::{fs, path::PathBuf};
use structopt::StructOpt;
use simple_logger::SimpleLogger;
use anyhow::{
	Error,
	Result,
	bail,
	ensure
};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref VERBOSE: RwLock<bool> = RwLock::new(false);
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
	/// Verbose mode.
	#[structopt(short, long)]
	verbose: bool,

	/// File to process.
	#[structopt(name = "INPUT", parse(from_os_str))]
	input: PathBuf,

	/// Output file.
	#[structopt(name = "OUTPUT", parse(from_os_str))]
	output: PathBuf,
}

fn main() -> Result<()> {
	SimpleLogger::new().init().unwrap();
	let opt: Opt = Opt::from_args();
	{ *VERBOSE.write().unwrap() = opt.verbose; }

	let input = fs::read_to_string(&opt.input).expect("Failed reading the input file.");
	verbose!("output: {:#?}", &input);

	let output = fs::read_to_string(&opt.input).expect("Failed reading the output file.");
	verbose!("output_dir {:#?}", &output);

	fs::write(output, "Hello, world!").expect("Unable to write file");
	log::info!("Hello, world!");
	Ok(())
}

#[macro_export]
macro_rules! verbose {
    ($target:literal, $($arg:tt)+) => {
   		{ if *VERBOSE.read().unwrap() { log::info!($target, $($arg)+); } }
    };
}
