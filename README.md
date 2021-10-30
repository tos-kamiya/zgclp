# zgclp

Zgclp (Zero-grammer definition command-line parser) is one of Rust's command-line parsers. 
A normal command-line parser generates a parser from the definition of command-line options that accepts a command-line according to its grammar. In contrast, zgclp uses a universal parser to discover what it assumes to be options and or arguments from the given command line arguments.

## How it works?

Do a `cargo build` and run `target/debug/zgclp` with something like command line arguments.

```sh
> target/debug/zgclp foo -bar bal --bra boo 
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

`-a bc`.

zgclp will output the following two interpretations.

* The option `-a` appears with no arguments (the next `bc` is a normal command line argument that has nothing to do with the option `-a`).
* The option `-a` appears with the argument `bc`.

## How do I use zgclp?

See a sample code: `src/main.rs`.

