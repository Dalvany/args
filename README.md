# args

A dead simple implementation of command line argument parsing and validation
built on top of the [getopts](https://crates.io/crates/getopts) crate.

In order to use the `args` crate simply create an `Args` object and begin
registering possible command line options via the `flag(...)` and `option(...)`
methods. Once all options have been registered, parse arguments directly from the
command line, or provide a vector of your own arguments.

If any errors are encountered during parsing the method will panic, otherwise,
arguments can be retrieved from the `args` instance by calling `value_of(...)`
or `validated_value_of(...)`.

That's it!

## Usage

This crate is [on crates.io](https://crates.io/crates/args) and can be
used by adding `args` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
args = "0.1"
```

and this to your crate root:

```rust
extern crate args;
```

## Example

The following example shows simple command line parsing for an application that
requires a number of iterations between zero *(0)* and ten *(10)* to be specified,
accepts an optional log file name and responds to the help flag.

```{.rust}
extern crate args;
extern crate getopts;

use args::{Args,ArgsError,Order,OrderValidation};
use getopts::Occur;

const PROGRAM_NAME: &'static str = "program";

fn main() {
    parse(&vec!("-i", "15"));
}

fn parse(input: &Vec<&str>) -> Result<(), ArgsError> {
    let mut args = Args::new(PROGRAM_NAME);
    args.flag("h", "help", "Print the usage menu");
    args.option("i",
        "iter",
        "The number of times to run this program",
        "TIMES",
        Occur::Req,
        None);
    args.option("l",
        "log_file",
        "The name of the log file",
        "NAME",
        Occur::Optional,
        None);

    try!(args.parse(input));

    let help = try!(args.value_of("help"));
    if help {
        args.full_usage(&format!("How to use {}", PROGRAM_NAME));
        return Ok(());
    }

    let gt_0 = Box::new(OrderValidation::new(Order::GreaterThan, 0u32));
    let lt_10 = Box::new(OrderValidation::new(Order::LessThanOrEqual, 10u32));

    let iters = try!(args.validated_value_of("iter", &[gt_0, lt_10]));
    for iter in 0..iters {
        println!("Working on iteration {}", iter);
    }
    println!("All done.");

    Ok(())
}
```rust
