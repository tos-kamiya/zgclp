// run with `cargo run --example zgclp_boilerplate -- ....`, e.g.:
// cargo run --example zgclp_boilerplate -- -h
// cargo run --example zgclp_boilerplate -- -n 1

use std::env;

use zgclp::{arg_parse_ahv, Arg};

const DOC: &'static str = "Zgclp demonstration.

Usage:
  zgclp_boilerplate [options] [--] <arguments>...

Options:
  --help, -h        Show this message.
  --version, -v     Show version info.
  ....
";

fn main() {
    let argv_store: Vec<String> = env::args().collect();
    let argv: Vec<&str> = argv_store.iter().map(AsRef::as_ref).collect();

    let mut args = Vec::<&str>::new();

    // ** Sample options **
    let mut output_file = None;
    let mut dry_run = false;

    let mut arg_index = 1;
    while arg_index < argv.len() {
        let eat = match arg_parse_ahv(&argv, arg_index, &mut args, DOC) {
            // ** Sample option (with argument) **
            (Arg::Option("-o" | "--output"), _, Some((eat, value))) => {
                output_file = Some(value);
                eat
            }
            // ** Sample option (w/o argument) **
            (Arg::Option("-n" | "--dry-run"), Some(eat), _) => {
                dry_run = true;
                eat
            }

            // Skip arguments processed by zgclp / Error handling
            (Arg::Processed, Some(eat), None) => {
                eat
            }
            (Arg::Option(name), _, _) => {
                eprintln!("Error: unknown option, or option missing argument: {}", name);
                std::process::exit(1);
            }
            _ => {
                panic!("Internal error in command-line parsing.");
            }
        };
        arg_index += eat;
    }

    // ** Sample stuff **
    if let Some(n) = output_file {
        println!("output_file = {}", n);
    }
    println!("dry_run: {:?}", dry_run);
    println!("args = {:?}", args);
}
