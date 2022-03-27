// A sample main function which parse command-line arguments with zgclp.

use std::env;

use zgclp::{arg_parse, Arg};

fn main() {
    let argv_store: Vec<String> = env::args().collect();
    let argv: Vec<&str> = argv_store.iter().map(AsRef::as_ref).collect();
    let mut arg_index = 1;
    if argv.len() == 1 {
        println!("No arguments.");
        return;
    }
    while arg_index < argv.len() {
        let eat = match arg_parse(&argv, arg_index) {
            (Arg::Option(name), _, Some((eat, value))) => {
                // if the option can be interpreted as having an argument (i)
                println!("Option \"{}\" with argument \"{}\" .", name, value);
                eat
            }
            (Arg::Option(name), Some(eat), _) => {
                // if the option can be interpreted as having NO arguments (ii)
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

// In this program, the case where an option can be interpreted as having an argument (i) is given
// priority over the case where it can be interpreted as not having an argument (ii).
// Therefore, if an option can be interpreted as either (i) or (ii), it is interpreted as (i).
//  The order of priority can be changed by flipping the order of clauses (i) and (ii) in the program.
