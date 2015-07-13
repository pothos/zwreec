# zwreec [![Build Status](https://travis-ci.org/Drakulix/zwreec.svg?branch=master)](https://travis-ci.org/Drakulix/zwreec)

<img width=144px src="https://dl.dropboxusercontent.com/u/70410095/zwreec/logo.png">
*Logo by [@madmalik](https://github.com/madmalik)*

zwreec is a compiler for [interactive fiction](http://en.wikipedia.org/wiki/Interactive_fiction) in the Twee format (created by the [Twine software](http://en.wikipedia.org/wiki/Twine_(software))) to [Z-Machine](http://en.wikipedia.org/wiki/Z-machine) instructions (Zcode) which can be run with interpreters like [frotz](http://frotz.sourceforge.net) or [gargoyle](http://ccxvii.net/gargoyle/).

This project is build using the [Rust](http://www.rust-lang.org) language and [Cargo](http://doc.crates.io) as package manager. It is structured as a library, so that it can be used by other Rust projects, but it also contains a reference binary implementation, which can be used as a fully functioning compiler.

## Getting zwreec

### Downloads

You can [download](https://github.com/Drakulix/zwreec/releases) a prebuilt version of our reference implementation for 64bit Linux and OS X.

### Compiling from Source

To build zwreec from Source, you will need to have both Rust 1.1.0 and Cargo installed on your system. You can download the Rust binarys on their [website](http://www.rust-lang.org/install.html), by using your system's package manager or by running this in your shell:

```sh
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

Cargo should be installed alongside current Rust binaries.

After installing Rust, zwreec can be complied using these commands:

```sh
git clone https://github.com/Drakulix/zwreec
cd zwreec
cargo build --release
```

The resulting binary can be found at `target/release/zwreec`.

### Usage

```
Usage: zwreec [-hV] [-vqwf] [-l [LOGFILE]] [-o OUTPUT] INPUT

Options:
    -v --verbose        Be more verbose. Can be used multiple times.
    -q --quiet          Be quiet
    -w --overwrite      Overwrite output file if necessary.
    -l --logfile [LOGFILE]
                        Specify log file (additionally to logging on stderr)
    -o FILE             Name of the output file
    -h --help           Display this help and exit
    -V --version        Display version

Additional help:
    --help -v           Print the full set of options zwreec accepts
```

#### Compiling a sample Twee file

This repository contains a few sample Twee-Files used for the library's integration tests. They are located under `tests/integration/should-compile/`. To compile them you can use the following command:

```
$ ./target/release/zwreec -o CurrentStatus.z8 ./tests/integration/should-compile/CurrentStatus.twee
```

_Edit the above line to compile different twee adventures._

Then you can run `./CurrentStatus.z8` with your favorite Z-Code interpreter.

## Using zwreec as a library

The library uses a [fork of rustlex](https://github.com/Drakulix/rustlex) to to do lexical analysis.

To use zwreec in your project you can add it as a dependency to your `Cargo.toml`.

```toml
[dependencies.zwreec]
git = "https:://github.com/Drakulix/zwreec"
```

Then you can use it in your crate root:

```rust
extern crate zwreec;
```

### Example

The following example is itself a full working command line compiler. It is a simpler
version of the [Reference Binary Implementation](#getting-zwreec).

```rust
extern crate zwreec;

use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 { panic!("Need exactly one input file!"); }

    let cfg = zwreec::config::Config::default_config();
    let mut input = match File::open(Path::new(&args[1])) {
        Ok(file) => file,
        Err(why) => { panic!("Couldn't open input: {}", Error::description(&why)); }
    };
    let mut output = match File::create(Path::new("a.z8")) {
        Ok(file) => file,
        Err(why) => { panic!("Couldn't open output: {}", Error::description(&why)); }
    };

    zwreec::compile(cfg, &mut input, &mut output);
}
```

## Development and Documentation

zwreec was developed by students as part of the course "Softwareprojekt Übersetzerbau" in 2015. 

You can find the extensive autogenerated documentation of zwreec and its dependencies under [drakulix.github.io/zwreec/](https://drakulix.github.io/zwreec/).
