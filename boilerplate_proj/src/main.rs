use std::env;

use zgclp::{arg_parse, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
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
    let mut output_file = None; // a sample option, with argument
    let mut dry_run = false; // a sample option, w/o argument

    let mut arg_index = 1;
    while arg_index < argv.len() {
        let eat = match arg_parse(&argv, arg_index) {
            (Arg::Option("-h" | "--help"), Some(_eat), _) => {
                println!("{}", DOC);
                std::process::exit(0);
            }
            (Arg::Option("-v" | "--version"), Some(_eat), _) => {
                println!("{} {}", NAME, VERSION);
                std::process::exit(0);
            }

            // // Option with argument
            // (Arg::Option(OPTION_NAME), _, Some((eat, value))) => {
            //     ....
            //     eat
            // }

            // // Option w/o argument
            // (Arg::Option(OPTION_NAME), Some(eat), _) => {
            //     ....
            //     eat
            // }

            // Sample options
            (Arg::Option("-o" | "--output"), _, Some((eat, value))) => {
                output_file = Some(value);
                eat
            }
            (Arg::Option("-n" | "--dry-run"), Some(eat), _) => {
                dry_run = true;
                eat
            }

            // Argument
            (Arg::Value, None, Some((eat, value))) => {
                args.push(value);
                eat
            }

            // Separator
            (Arg::Separator(_name), Some(_eat), None) => {
                args.extend(&argv[arg_index + 1..]);
                argv.len() - arg_index
            }

            // Unknown option/parse error
            (Arg::Option(name), _, _) => {
                eprintln!("Error: unknown option: {}", name);
                std::process::exit(1);
            }
            _ => {
                panic!("Internal error in command-line argument parsing.");
            }
        };
        arg_index += eat;
    }

    // Do stuff
    if let Some(n) = output_file {
        println!("output_file = {}", n);
    }
    println!("dry_run: {:?}", dry_run);
    println!("args = {:?}", args);
}
