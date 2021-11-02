// run with `cargo run --example zgclp_boilerplate_with_debugging_support -- ....`, e.g.:
// cargo run --example zgclp_boilerplate_with_debugging_support -- -h
// cargo run --example zgclp_boilerplate_with_debugging_support -- -n 1 xyz

use std::env;

use zgclp::{arg_parse_ahv, Arg};
// use zgclp::{arg_parse, Arg}; // debug, in case you want to examine the behavior of arg_parse in more detail

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

    let mut parse_results: Vec<(Arg, Option<usize>, Option<(usize, &str)>)> = Vec::new(); // debug

    let mut arg_index = 1;
    while arg_index < argv.len() {
        let pr = arg_parse_ahv(&argv, arg_index, &mut args, DOC);
        // let pr = arg_parse(&argv, arg_index); // debug, in case you want to examine the behavior of arg_parse in more detail
        parse_results.push(pr); // debug
        let eat = match pr {
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

            // debug, in case you want to examine the behavior of arg_parse in more detail
            (Arg::Value, None, Some((eat, _value))) => {
                eat
            }
            _ => {
                panic!("Internal error in command-line parsing.");
            }
        };
        arg_index += eat;
    }

    // debug
    println!("parse_results:");
    for (i, pr) in parse_results.iter().enumerate() {
        println!("[{}] {:?}", i, pr);
    }

    // ** Sample stuff **
    if let Some(n) = output_file {
        println!("output_file = {}", n);
    }
    println!("dry_run: {:?}", dry_run);
    println!("args = {:?}", args);
}
