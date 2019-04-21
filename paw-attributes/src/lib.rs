//! Macros for the paw crate.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.decl.output;
    let name = &input.ident;
    let body = &input.block;
    let asyncness = &input.asyncness;

    if name != "main" {
        let tokens = quote_spanned! { name.span() =>
            compile_error!("only the main function can be tagged with #[paw::main]");
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
            let pat = &arg.pat;
            let ty = &arg.ty;
            quote! {
                #asyncness fn main() #ret {
                    __validate(&pat);
                    fn main (#pat: #ty::Item) {
                        #body
                    }

                    main(&pat.parse())
                }
                fn __validate(arg: &impl PawTrait) -> #ty {
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
