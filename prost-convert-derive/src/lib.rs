//! This crate provides a derive macro for `TryFromProto` and `TryFromProto` traits.
//!

// TODO: change it to `warn(..)` if we go open source. Indeed `deny(..)` could break user code if it uses a
// newer version of rust with new warnings)
#![deny(clippy::all, clippy::cargo, missing_docs)]

mod attributes;
mod container;
mod expand;
mod symbol;
use expand::expand_derive_prost_convert;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Expand the derive macro.
#[proc_macro_derive(ProstConvert, attributes(prost_convert))]
pub fn derive_prost_convert(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input as syn::DeriveInput);
    expand_derive_prost_convert(ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
