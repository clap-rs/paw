//! Macros for the paw crate.

#![recursion_limit = "512"]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    unimplemented!();
}
