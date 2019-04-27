//! Traits for the [Paw crate].
//!
//! [Paw crate]: https://docs.rs/paw

#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]

/// Allow a type to be parsed as arguments to main.
pub trait ParseArgs: Sized {
    /// Error type.
    ///
    /// This error type needs to be compatible with the `Result` type returned by `fn main`.
    type Error;

    /// Try to create a new instance of the struct.
    ///
    /// Returns a `Result` of either an instance of `Self`, or the associated `Error` type.
    fn parse_args() -> Result<Self, Self::Error>;
}
