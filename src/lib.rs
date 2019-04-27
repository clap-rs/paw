#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![warn(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//!  Command line argument paw-rser abstraction for main.
//! ## Example
//!
//! ```rust
//! ```

use std::env::{Args as StdArgs, ArgsOs as StdArgsOs};
use std::iter::Iterator;
use std::ffi::OsString;
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
    fn len(&self) -> usize { self.inner.len() }
}

impl DoubleEndedIterator for ArgsOs {
    fn next_back(&mut self) -> Option<OsString> {
        self.inner.next_back()
    }
}

impl ParseArgs for ArgsOs {
    type Error = std::io::Error;
    fn try_parse() -> Result<Self, Self::Error> {
        Ok(Self { inner: std::env::args_os() })
    }
}
