//! Structopt adapter for the Paw crate.
//!
//! ## Examples
//! ```
//! use std::io::prelude::*;
//! use std::net::TcpListener;
//!
//! #[derive(paw_structopt::StructOpt, structopt::StructOpt)]
//! struct Args {
//!     /// Port to listen on.
//!     #[structopt(short = "p", long = "port", env = "PORT", default_value = "8080")]
//!     port: u16,
//!     /// Address to listen on.
//!     #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
//!     address: String,
//! }
//!
//! #[paw::main]
//! fn main(args: Args) -> Result<(), std::io::Error> {
//!     let listener = TcpListener::bind((args.address.as_str(), args.port))?;
//!     println!("listening on {}", listener.local_addr()?);
//!     for stream in listener.incoming() {
//!         stream?.write(b"hello world!")?;
//!     }
//!     Ok(())
//! }
//! ```

#![recursion_limit = "512"]
#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructOpt)]
pub fn structopt(input: TokenStream) -> TokenStream {
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
