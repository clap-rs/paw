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
```rust
#[paw::main]
fn main(args: paw::Args) {
    for arg in args {
        println!("{:?}", arg);
    }
}
```

__More Examples__
- [Scratch](https://github.com/rust-cli/paw/tree/master/examples/scratch.rs)
- [Args](https://github.com/rust-cli/paw/tree/master/examples/args.rs)
- [Structopt](https://github.com/rust-cli/paw/tree/master/examples/structopt.rs)

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
### How does paw work?
The `paw::main` attribute allows `fn main` to take any argument that implements the `paw::ParseArgs`
trait. `paw::ParseArgs` implements one method: `parse_args` which returns a `Result<Self>`.

Any errors that are generated are returned back from `fn main`, and the returned `Result` type is in
control of how to print them.

### What is the relationship to C?
In C's runtime, arguments command line arguments are passed as a combination of "number of
arguments" (`argc`), and a "list of arguments" (`argv`):
```c
int main(int argc, char **argv) {
    for(i = 1; i < argc; i++) {
        printf("%s",argv[i]);
    }
    return 0;
}
```

In Rust this would translate to an iterator of arguments, which is what
[`std::env::Args`](https://doc.rust-lang.org/std/env/struct.Args.html) provides, which is wrapped in
`paw` through [`paw::Args`](https://docs.rs/paw/target/doc/paw/struct.Args.html).

### What's the future for paw?
Paw is an experiment by the CLI WG to provide a better command line experience for everyone. Our
hypothesis is that by moving command line parsing to `fn main` Rust's command line experience can
become more intuitive and easy to use.

We hope to gather feedback how this works for people, and see how we can integrate with existing
libraries. This will take time, and we might change things in the process.

If this experiment proves to be successful, we might move to formalize the features Paw provide into
Rust itself through the RFC process. But that's not the case yet, so for now: we hope you enjoy Paw,
and we'd love to hear your about your experiences using it!

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
