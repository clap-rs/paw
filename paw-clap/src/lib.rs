//! Clap adapter for the Paw crate.

#![recursion_limit = "512"]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Paw)]
pub fn paw(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let result = quote! {
        impl paw::ParseArgs for #name {
            /// Error type.
            type Error = std::io::Error;

            /// Try to parse an input to a type.
            fn parse_args() -> Result<Self, Self::Error> {
                Ok(<#name as structopt::StructOpt>::from_args())
            }
        }
    };

    result.into()
}
