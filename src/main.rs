extern crate getopts;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate csv;

use std::{env, process};
use std::fs::File;
use std::io::{Write, BufWriter};

use getopts::Options;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod rnacentral;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("{} version {}\n\nUsage: {} [options]", PKG_NAME, VERSION, program);
    eprint!("{}", opts.usage(&brief));
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help message");
    opts.optopt("r", "rfam-annotations-file", "Path to rfam_annotations.tsv.gz",
                "FILE");
    opts.optopt("i", "identifer-file", "Path to pombase.tsv from RNAcentral FTP site",
                "FILE");
    opts.optopt("o", "output-file", "Output JSON file",
                "FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("Invalid options\n{}", f)
    };

    let program = args[0].clone();

    if matches.opt_present("help") {
        print_usage(&program, opts);
        process::exit(0);
    }

    if !matches.opt_present("rfam-annotations-file") {
        print!("no -r|--rfam-annotations-file option\n");
        print_usage(&program, opts);
        process::exit(1);
    }

    if !matches.opt_present("identifer-file") {
        print!("no -i|--identifer-file option\n");
        print_usage(&program, opts);
        process::exit(1);
    }

    if !matches.opt_present("output-file") {
        print!("no -o|--output-file option\n");
        print_usage(&program, opts);
        process::exit(1);
    }

    let rfam_annotations_filename = matches.opt_str("r").unwrap();
    let identifier_filename = matches.opt_str("i").unwrap();
    let output_filename = matches.opt_str("o").unwrap();

    match rnacentral::parse(&identifier_filename, &rfam_annotations_filename) {
        Ok(res) => {
            let json: String = serde_json::to_string(&res)?;
            let out_file = File::create(output_filename).expect("Unable to open file");
            let mut out_writer = BufWriter::new(&out_file);
            out_writer.write_all(json.as_bytes()).expect("Unable to write!");

            Ok(())
        },
        Err(message) => {
            eprint!("failed with error: {}", message);
            process::exit(1);
        }
    }
}
