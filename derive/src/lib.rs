extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, MetaList, NestedMeta};

const DERIVE_CONFIG_ERROR: &str =
    "The config filename must be specified using [config(filename = \"config.json\")]";

// TODO: Make this stricter
#[proc_macro_derive(Config, attributes(config))]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Find #[config(...)] attribute (should only be one)
    let meta = &input
        .attrs
        .iter()
        .find(|a| a.path.is_ident("config"))
        .expect(DERIVE_CONFIG_ERROR)
        .parse_meta()
        .expect(DERIVE_CONFIG_ERROR);

    // Make sure it's of kind #[config(key = "value")]
    let nested_meta = if let Meta::List(MetaList { nested: n, .. }) = meta {
        n.first().expect(DERIVE_CONFIG_ERROR)
    } else {
        panic!(DERIVE_CONFIG_ERROR);
    };
    let meta_name_value = if let NestedMeta::Meta(Meta::NameValue(mnv)) = nested_meta {
        mnv
    } else {
        panic!(DERIVE_CONFIG_ERROR);
    };

    // Make sure it's of kind #[config(filename = "value")]
    if !meta_name_value.path.is_ident("filename") {
        panic!(DERIVE_CONFIG_ERROR);
    }

    // Get value
    let f = &meta_name_value.lit;

    let expanded = quote! {
        impl symphony::config::Config for #name {
            fn filepath() -> &'static str {
                concat!("/sdcard/Android/data/com.beatgames.beatsaber/files/mod_cfgs/", #f)
            }
        }
    };
    expanded.into()
}
