#![doc = include_str!("../README.md")]

use std::env;

fn rstrip(s: &str) -> &str {
    let mut tail_spaces = 0;
    let mut chars = s.chars();
    loop {
        if let Some(last_char) = chars.next_back() {
            if last_char == '\n' || last_char == ' ' || last_char == '\t' {
                tail_spaces += 1;
                continue;
            }
        }
        break;
    }
    &s[0..s.len() - tail_spaces]
}

/// An enum type that indicates whether the parsing result is a command-line argument,
/// an option, a separator, or an item that should be skipped because the parser has processed it.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Arg<'a> {
    Value,              // (usual) argument
    Option(&'a str),    // option
    Separator(&'a str), // separator
    Processed, // should be skipped (because already processed. generated when `arg_parse_a` or `arg_parse_ahv` is called)
}

macro_rules! some_pair {
    ( $v1:expr, $v2:expr ) => {
        Some(($v1, $v2))
    };
}

pub type ArgParseResult<'s> = (Arg<'s>, Option<usize>, Option<(usize, &'s str)>);

/// Function to parse command line arguments.
/// To use this function, pass the string array of command-line arguments (`argv: &'a [&'s str]`) and
/// the position to start parsing (`index: usize`).  
/// The return value is a tuple with three values (`(Arg<'s>, Option<usize>, Option<(usize, &'s str)>)`).
/// The first value indicates whether the result of the parse is an option or a normal argument, etc.   
/// The second value indicates the increment to the next parse start position if the result is
/// interpreted as an option with no arguments, otherwise None.  
/// The third value is the increment to the next parsing start position and the argument string,
/// if the parsing result is interpreted as an option with arguments. Otherwise, None.  
pub fn arg_parse<'s, 'a>(argv: &'a [&'s str], index: usize) -> ArgParseResult<'s> {
    let a = argv[index];
    if a == "-" {
        (Arg::Value, None, some_pair!(1, a))
    } else if a == "--" {
        (Arg::Separator(a), Some(1), None)
    } else if a.starts_with("--") {
        if let Some(i) = a.find('=') {
            (Arg::Option(&a[..i]), None, some_pair!(1, &a[i + 1..]))
        } else if index + 1 < argv.len() {
            let a2 = argv[index + 1];
            if a2 == "-" || !a2.starts_with('-') {
                (Arg::Option(a), Some(1), some_pair!(2, a2))
            } else {
                (Arg::Option(a), Some(1), None)
            }
        } else {
            (Arg::Option(a), Some(1), None)
        }
    } else if a.starts_with('-') {
        if a.len() > 2 {
            (Arg::Option(&a[..2]), None, some_pair!(1, &a[2..]))
        } else if index + 1 < argv.len() {
            let a2 = argv[index + 1];
            if a2 == "-" || !a2.starts_with('-') {
                (Arg::Option(a), Some(1), some_pair!(2, a2))
            } else {
                (Arg::Option(a), Some(1), None)
            }
        } else {
            (Arg::Option(a), Some(1), None)
        }
    } else {
        (Arg::Value, None, some_pair!(1, a))
    }
}

/// Almost the same as arg_parse, but collects the arguments.
pub fn arg_parse_a<'s, 'a, 'b>(
    argv: &'a [&'s str],
    index: usize,
    args: &'b mut Vec<&'s str>,
) -> ArgParseResult<'s> {
    let (a, na, wa) = arg_parse(argv, index);
    match a {
        Arg::Value => {
            if let Some((eat, value)) = wa {
                args.push(value);
                (Arg::Processed, Some(eat), None)
            } else {
                panic!("zgclp internal error (arg_parse_a Arg::Value)")
            }
        }
        Arg::Separator(_name) => {
            args.extend(&argv[index + 1..]);
            (Arg::Processed, Some(argv.len() - index), None)
        }
        _ => (a, na, wa),
    }
}

/// Helper function to handle the typical --help option.
pub fn handle_help_option<'s, 't>(
    pr: ArgParseResult<'s>,
    help_message: &'t str,
) -> ArgParseResult<'s> {
    let (a, na, wa) = pr;
    match a {
        Arg::Option("-h" | "--help") => {
            println!("{}", rstrip(help_message));
            std::process::exit(0);
        }
        _ => (a, na, wa),
    }
}

/// Helper function to handle the typical --version/-v option.
pub fn handle_version_option(pr: ArgParseResult) -> ArgParseResult {
    let (a, na, wa) = pr;
    match a {
        Arg::Option("-v" | "--version") => {
            let version = env!("CARGO_PKG_VERSION");
            let name = env!("CARGO_PKG_NAME");
            println!("{} {}", name, version);
            std::process::exit(0);
        }
        _ => (a, na, wa),
    }
}

/// Helper function to handle the typical --version/-V option.
pub fn handle_v_option(pr: ArgParseResult) -> ArgParseResult {
    let (a, na, wa) = pr;
    match a {
        Arg::Option("-V" | "--version") => {
            let version = env!("CARGO_PKG_VERSION");
            let name = env!("CARGO_PKG_NAME");
            println!("{} {}", name, version);
            std::process::exit(0);
        }
        _ => (a, na, wa),
    }
}

/// Almost the same as arg_parse, but collects the arguments and handles options --help/-h and --version/-v.
pub fn arg_parse_ahv<'s, 't, 'a, 'b>(
    argv: &'a [&'s str],
    index: usize,
    args: &'b mut Vec<&'s str>,
    help_message: &'t str,
) -> ArgParseResult<'s> {
    let pr = arg_parse_a(argv, index, args);
    let pr2 = handle_help_option(pr, help_message);
    handle_version_option(pr2)
}

/// Almost the same as arg_parse, but collects the arguments and handles options --help/-h and --version/-V.
pub fn arg_parse_ah_v<'s, 't, 'a, 'b>(
    argv: &'a [&'s str],
    index: usize,
    args: &'b mut Vec<&'s str>,
    help_message: &'t str,
) -> ArgParseResult<'s> {
    let pr = arg_parse_a(argv, index, args);
    let pr2 = handle_help_option(pr, help_message);
    handle_v_option(pr2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn short_option_simple() {
        let args = vec!["-a", "1", "-f", "-g3"];
        let v = arg_parse(&args, 0);
        assert_eq!(v, (Arg::Option("-a"), Some(1), some_pair!(2, "1")));
        let v = arg_parse(&args, 1);
        // eprintln!("{:?}", v);
        assert_eq!(v, (Arg::Value, None, some_pair!(1, "1")));
        let v = arg_parse(&args, 2);
        assert_eq!(v, (Arg::Option("-f"), Some(1), None));
        let v = arg_parse(&args, 3);
        assert_eq!(v, (Arg::Option("-g"), None, some_pair!(1, "3")));
    }

    #[test]
    fn short_option_complicated() {
        let args = vec!["-a=1", "-f", "-", "-g", "--", "-h"];
        let v = arg_parse(&args, 0);
        assert_eq!(v, (Arg::Option("-a"), None, some_pair!(1, "=1")));
        let v = arg_parse(&args, 1);
        assert_eq!(v, (Arg::Option("-f"), Some(1), some_pair!(2, "-")));
        let v = arg_parse(&args, 2);
        assert_eq!(v, (Arg::Value, None, some_pair!(1, "-")));
        let v = arg_parse(&args, 3);
        assert_eq!(v, (Arg::Option("-g"), Some(1), None));
        let v = arg_parse(&args, 4);
        assert_eq!(v, (Arg::Separator("--"), Some(1), None));
        let v = arg_parse(&args, 5);
        assert_eq!(v, (Arg::Option("-h"), Some(1), None));
    }

    #[test]
    fn long_option_simple() {
        let args = vec!["--aa", "1", "--ff", "--gg=3"];
        let v = arg_parse(&args, 0);
        assert_eq!(v, (Arg::Option("--aa"), Some(1), some_pair!(2, "1")));
        let v = arg_parse(&args, 1);
        // eprintln!("{:?}", v);
        assert_eq!(v, (Arg::Value, None, some_pair!(1, "1")));
        let v = arg_parse(&args, 2);
        assert_eq!(v, (Arg::Option("--ff"), Some(1), None));
        let v = arg_parse(&args, 3);
        assert_eq!(v, (Arg::Option("--gg"), None, some_pair!(1, "3")));
    }

    #[test]
    fn match_test() {
        let args = vec!["--aa", "1", "--bb=3"];
        let v = arg_parse(&args, 0);
        match v {
            (Arg::Option("--aa"), Some(1), Some((2, "1"))) => {}
            _ => {
                panic!("match fails.")
            }
        }
        match v {
            (Arg::Option("--aa"), _, Some((eat, "1"))) => {
                assert_eq!(eat, 2 as usize);
            }
            _ => {
                panic!("match fails.")
            }
        }

        let v = arg_parse(&args, 2);
        match v {
            (Arg::Option("--bb"), None, Some((eat, value))) => {
                assert_eq!(eat, 1 as usize);
                assert_eq!(value, "3");
            }
            _ => {
                panic!("match fails.")
            }
        }
    }

    #[test]
    fn argument_collection_simple() {
        let argv = vec!["foo", "--aa", "1", "--bb=2", "3", "-c4", "5"];

        let mut args = Vec::<&str>::new();
        let mut arg_index = 1;
        while arg_index < argv.len() {
            let eat = match arg_parse_a(&argv, arg_index, &mut args) {
                (Arg::Option(_name), _, Some((eat, _value))) => eat,
                (Arg::Option(_name), Some(eat), None) => eat,
                (Arg::Processed, Some(eat), _) => eat,
                _ => {
                    panic!("in argument_collection_simple")
                }
            };
            arg_index += eat;
        }
        assert_eq!(args, vec!["3", "5"]);
    }

    #[test]
    fn argument_collection_complicated() {
        let argv = vec!["foo", "--aa", "--bb=1", "2", "-c=3", "--", "-d", "4"];

        let mut args = Vec::<&str>::new();
        let mut arg_index = 1;
        while arg_index < argv.len() {
            let eat = match arg_parse_a(&argv, arg_index, &mut args) {
                (Arg::Option(_name), _, Some((eat, _value))) => eat,
                (Arg::Option(_name), Some(eat), None) => eat,
                (Arg::Processed, Some(eat), _) => eat,
                _ => {
                    panic!("in argument_collection_complicated")
                }
            };
            arg_index += eat;
        }
        assert_eq!(args, vec!["2", "-d", "4"]);
    }

    #[test]
    fn rstrip_test() {
        assert_eq!(rstrip("abc\n"), "abc");
        assert_eq!(rstrip("abc  "), "abc");
    }
}
