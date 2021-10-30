// A sample main function which parse command-line arguments with zgclp.

use std::env;

use zgclp::{arg_parse, Arg};

fn main() {
    let argv_store: Vec<String> = env::args().collect();
    let argv: Vec<&str> = argv_store.iter().map(AsRef::as_ref).collect();
    let mut arg_index = 1;
    while arg_index < argv.len() {
        let eat = match arg_parse(&argv, arg_index) {
            (Arg::Option(name), _, Some((eat, value))) => {
                println!("Option \"{}\" with argument \"{}\" .", name, value);
                eat
            }
            (Arg::Option(name), Some(eat), _) => {
                println!("Option \"{}\" w/o argument .", name);
                eat
            }
            (Arg::Value, None, Some((eat, value))) => {
                println!("Argument \"{}\" .", value);
                eat
            }
            (Arg::Separator(name), Some(eat), None) => {
                println!("Separator \"{}\" .", name);
                eat
            }
            _ => {
                panic!("Internal error in command-line argument parsing.");
            }
        };
        arg_index += eat;
    }
}
