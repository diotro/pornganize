#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#![warn(
    deprecated_in_future,
    keyword_idents,
    trivial_casts,
    unused_import_braces,
)]

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Model, attributes(model))]
pub fn model_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_model_derive(&ast).into()
}

fn impl_model_derive(ast: &syn::DeriveInput) -> TokenStream {
    dbg!(ast);
    TokenStream::new()
}
