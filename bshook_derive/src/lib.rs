extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{export::ToTokens, parse_macro_input, DeriveInput, Lit, Meta};

#[proc_macro_derive(Config, attributes(config))]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let filenames: Vec<proc_macro2::TokenStream> = input
        .attrs
        .into_iter()
        .filter_map(|a| a.parse_meta().ok())
        .filter_map(|m| match m {
            Meta::NameValue(nv) => Some(nv),
            _ => None,
        })
        .filter_map(|nv| match nv.lit {
            Lit::Str(s) => Some(s.into_token_stream()),
            _ => None,
        })
        .collect();

    if filenames.is_empty() {
        panic!("The config filename must be specified using [config = \"filename\"]");
    }
    let filename = &filenames[0];

    let expanded = quote! {
        impl bshook::config::Config for #name {
            fn filename() -> &'static str {
                #filename
            }
        }
    };
    expanded.into()
}
