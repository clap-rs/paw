# paw
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Command line argument paw-rser abstraction for main.

Paw's goal is to show that C's idea of passing arguments into main wasn't that
bad at all, but just needed a bit of oxidation to make it work with Rust.

Paw defines a trait, a proc macro, and an example implementation that when
combined allow you to pass fully parsed arguments to main. Gone is the need to
remember which methods to call in order to parse arguments in the CLI. Instead
paw makes command line parsing feel first-class

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Listen on a port__
```rust
use std::io::{self, prelude::*};
use std::net::TcpListener;

struct Args {
    port: u16,
    address: String,
}

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind((args.address.as_str(), args.port))?;
    println!("listening on {}", listener.local_addr()?);

    for stream in listener.incoming() {
        stream?.write(b"hello world!")?;
    }
    Ok(())
}

impl paw::ParseArgs for Args {
    type Error = Box<dyn std::error::Error>;

    fn parse_args() -> Result<Self, Self::Error> {
        let mut args = std::env::args().skip(1);

        let address = args
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the address arg is missing"))?;

        let port = args
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the port arg is missing"))?
            .parse()?;

        Ok(Self { address, port })
    }
}
```

To start the server do:
```sh
$ cargo run --example scratch -- localhost 8080
```

## Installation
```sh
$ cargo add paw
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## FAQ
### What's the future for paw?
It's currently just an experiment. But if this turns out to be something that
works really well for people, it's not out of the question that we might look to
standardize it. But that's a big if. Until then: we hope you enjoy Paw!

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/paw.svg?style=flat-square
[2]: https://crates.io/crates/paw
[3]: https://img.shields.io/travis/yoshuawuyts/paw/master.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/paw
[5]: https://img.shields.io/crates/d/paw.svg?style=flat-square
[6]: https://crates.io/crates/paw
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/paw

[releases]: https://github.com/yoshuawuyts/paw/releases
[contributing]: https://github.com/yoshuawuyts/paw/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/paw/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/paw/labels/help%20wanted
