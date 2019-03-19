# paw
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Command line argument paw-rser.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Basic usage__
```rust
#[paw_clap(verbosity, log, port)]
struct Args;

#[paw::main]
fn main(args: Args) -> Result<(), failure::Error> {
    let mut listener = args.listener()?;
    for stream in listener.incoming() {
        stream.write(b"hello world!")?;
    }
}
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

## References
None.

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
