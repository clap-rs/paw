#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![warn(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//!  Command line argument paw-rser abstraction for main.
//! ## Example
//!
//! ```rust
//! ```

#[doc(inline)]
#[cfg(not(test))] // NOTE: exporting main breaks tests, we should file an issue.
pub use paw_attributes::main;

/// Try parsing.
pub trait TryParse {
    /// Item returned.
    type Item;

    /// Error type.
    type Error;

    /// Try to parse an input to a type.
    fn try_parse() -> Result<Self::Item, Self::Error>;
}
