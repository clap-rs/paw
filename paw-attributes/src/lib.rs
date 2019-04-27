//! Macros for the paw crate.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.decl.output;
    let name = &input.ident;
    let body = &input.block;
    let asyncness = &input.asyncness;

    if name != "main" {
        let tokens = quote_spanned! { name.span() =>
            compile_error!("only fn main can be tagged with #[paw::main]");
        };
        return TokenStream::from(tokens);
    }

    if let syn::ReturnType::Default = ret {
        let tokens = quote_spanned! { name.span() =>
            compile_error!("fn main must specify a return type");
        };
        return TokenStream::from(tokens);
    }

    let inputs = &input.decl.inputs;
    let result = match inputs.len() {
        0 => {
            quote! {
                #asyncness fn main() #ret {
                    #body
                }
            }
        }
        1 => {
            let arg = match inputs.first().unwrap().into_value() {
                syn::FnArg::Captured(arg) => arg,
                _ => {
                    let tokens = quote_spanned! { inputs.span() =>
                        compile_error!("fn main should take a fully formed argument");
                    };
                    return TokenStream::from(tokens);
                }
            };
            let arg_name = &arg.pat;
            let arg_type = &arg.ty;
            quote! {
                #asyncness fn main() #ret {
                    let #arg_name = <#arg_type as paw::ParseArgs>::parse_args()?;
                    #body
                }

            }
        }
        _ => {
            let tokens = quote_spanned! { inputs.span() =>
                compile_error!("fn main can take 0 or 1 arguments");
            };
            return TokenStream::from(tokens);
        }
    };

    result.into()
}
