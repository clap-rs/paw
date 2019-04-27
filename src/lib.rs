#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![warn(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//!  Command line argument paw-rser abstraction for main.
//! ## Example
//!
//! ```rust
//! ```

use std::env::Args as EnvArgs;
use std::iter::Iterator;
use std::fmt;

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
    inner: EnvArgs,
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
    fn len(&self) -> usize { self.inner.len() }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<String> {
        self.inner.next_back()
    }
}

impl ParseArgs for Args {
    type Error = std::io::Error;
    fn try_parse() -> Result<Self, Self::Error> {
        Ok(Self { inner: std::env::args() })
    }
}
