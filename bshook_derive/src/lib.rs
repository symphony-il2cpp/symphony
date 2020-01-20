extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta};

#[proc_macro_derive(Config, attributes(config))]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let filenames: Vec<Literal> = input
        .attrs
        .into_iter()
        .filter_map(|a| a.parse_meta().ok())
        .filter_map(|m| match m {
            Meta::NameValue(nv) => Some(nv),
            _ => None,
        })
        .filter_map(|nv| match nv.lit {
            Lit::Str(s) => s.parse::<Literal>().ok(),
            _ => None,
        })
        .collect();

    if filenames.is_empty() {
        compile_error!("The config filename must be specified using [config = \"filename\"]");
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
