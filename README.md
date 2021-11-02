# zgclp

Zgclp (Zero-grammar definition command-line parser) is one of Rust's command-line parsers. 
A normal command-line parser generates a parser from the definition of command-line options that accepts a command line according to its grammar. In contrast, zgclp uses a universal parser to discover what it assumes to be options or arguments from the given command-line arguments.

## How it works?

You can build the sample program with "cargo build" and try it as follows:

```sh
$ cargo build
$ target/debug/zgclp foo -bar bal --bra boo
Argument "foo" .
Option "-b" with argument "ar" .
Argument "bal" .
Option "--bra" with argument "boo" .
```

## Format of options accepted by zgclp

When you write a single letter of the alphabet as A, B, etc., zgclp accepts the following options.

|  Format   |                Parsed                 |
| --------- | ------------------------------------- |
| `-A`      | Option with no argument `-A`.         |
| `-A BC`   | Option `-A` with argument `BC`.       |
| `-ABC`    | Option `-A` with argument `BC`.       |
| `--AB`    | Option with no arguments `--AB`.      |
| `--AB CD` | Option `--AB` with the argument `CD`. |
| `--AB=CD` | Option `--AB` with argument `CD`.     |
| `--`      | Separator.                            |

"But isn't that ambiguous?" If you are wondering, you are correct.

When the command line is

`-a bc`

zgclp will output the following two interpretations.

* The option `-a` appears with no arguments (the next `bc` is a normal command-line argument that has nothing to do with the option `-a`).
* The option `-a` appears with the argument `bc`.

## How do I use zgclp?

**Short Answer:**

Copy the boilerplate code [examples/zgclp_boilerplate.rs](examples/zgclp_boilerplate.rs) as your `main.rs` and modify it.

**Long Answer:**

1. Call the function `arg_parse`, giving the command-line arguments as an array of strings (`&[&str]`) and the starting position of parsing.

2. The return value is a tuple with three values. 

* The first value indicates whether the result of the parse is an option or a normal argument, etc. 
* The second value indicates the increment to the next parse start position if the result is interpreted as an option with no arguments, otherwise None. 
* The third value is the increment to the next parsing start position and the argument string, if the parsing result is interpreted as an option with arguments. Otherwise, None.

Use `arg_parse` to do a "full" parsing of options and (normal) arguments, including the order of their appearance.
If you don't need such full parsing and it is enough to just get the values for normal arguments, consider using `arg_parse_a` or `arg_parse_ahv`.

See a sample code [src/main.rs](src/main.rs) for `arg_parse` or a boilerplate [examples/zgclp_boilerplate.rs](examples/zgclp_boilerplate.rs) for `arg_parse_ahv`.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Links

* [zgclp's page in Crates.io](https://crates.io/crates/zgclp)
* [zgclp's repository in GitHub](https://github.com/tos-kamiya/zgclp)
