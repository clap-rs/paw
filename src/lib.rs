#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![warn(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! Command line argument paw-rser abstraction for main.
//!
//! Paw's goal is to show that C's idea of passing arguments into main wasn't that
//! bad at all, but just needed a bit of oxidation to make it work with Rust.
//!
//! Paw defines a trait, a proc macro, and an example implementation that when
//! combined allow you to pass fully parsed arguments to main. Gone is the need to
//! remember which methods to call in order to parse arguments in the CLI. Instead
//! paw makes command line parsing feel first-class
//!
//! ## Example
//!
//! ```rust
//! use std::io::{self, prelude::*};
//! use std::net::TcpListener;
//!
//! struct Args {
//!     port: u16,
//!     address: String,
//! }
//!
//! #[paw::main]
//! fn main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
//!     let listener = TcpListener::bind((args.address.as_str(), args.port))?;
//!     println!("listening on {}", listener.local_addr()?);
//!
//!     for stream in listener.incoming() {
//!         stream?.write(b"hello world!")?;
//!     }
//!     Ok(())
//! }
//!
//! impl paw::ParseArgs for Args {
//!     type Error = Box<dyn std::error::Error>;
//!
//!     fn parse_args() -> Result<Self, Self::Error> {
//!         let mut args = std::env::args().skip(1);
//!
//!         let address = args
//!             .next()
//!             .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the address arg is missing"))?;
//!
//!         let port = args
//!             .next()
//!             .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the port arg is missing"))?
//!             .parse()?;
//!
//!         Ok(Self { address, port })
//!     }
//! }
//! ```
//!
//! To start the server do:
//! ```sh
//! $ cargo run --example scratch -- localhost 8080
//! ```
//!
//! __More Examples__
//! - [Scratch](https://github.com/rust-cli/paw/tree/master/examples/scratch.rs)
//! - [Args](https://github.com/rust-cli/paw/tree/master/examples/args.rs)
//! - [Structopt](https://github.com/rust-cli/paw/tree/master/examples/structopt.rs)

use std::env::{Args as StdArgs, ArgsOs as StdArgsOs};
use std::ffi::OsString;
use std::fmt;
use std::iter::Iterator;

#[doc(inline)]
#[cfg(not(test))] // NOTE: exporting main breaks tests, we should file an issue.
pub use paw_attributes::main;

#[doc(inline)]
pub use paw_raw::ParseArgs;

/// An iterator over the arguments of a process, yielding an [`String`] value for each argument.
///
/// This is a wrapper over [`std::env::Args`] which is an iterator over the arguments to a process.
/// See its documentation for more.
///
/// [`String`]: ../string/struct.String.html
/// [`std::env::Args`]: https://doc.rust-lang.org/std/env/struct.Args.html
pub struct Args {
    inner: StdArgs,
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Iterator for Args {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<String> {
        self.inner.next_back()
    }
}

impl ParseArgs for Args {
    type Error = std::io::Error;
    fn parse_args() -> Result<Self, Self::Error> {
        Ok(Self {
            inner: std::env::args(),
        })
    }
}

/// An iterator over the arguments of a process, yielding an [`OsString`] value for each argument.
///
/// This is a wrapper over [`std::env::ArgsOs`] which is an iterator over the arguments to a
/// process. See its documentation for more.
///
/// [`OsString`]: ../ffi/struct.OsString.html
/// [`std::env::ArgsOs`]: https://doc.rust-lang.org/std/env/struct.ArgsOs.html
pub struct ArgsOs {
    inner: StdArgsOs,
}

impl fmt::Debug for ArgsOs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Iterator for ArgsOs {
    type Item = OsString;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl ExactSizeIterator for ArgsOs {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl DoubleEndedIterator for ArgsOs {
    fn next_back(&mut self) -> Option<OsString> {
        self.inner.next_back()
    }
}

impl ParseArgs for ArgsOs {
    type Error = std::io::Error;
    fn parse_args() -> Result<Self, Self::Error> {
        Ok(Self {
            inner: std::env::args_os(),
        })
    }
}
