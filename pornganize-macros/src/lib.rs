#![allow(unused_imports, unreachable_code, dead_code, unused_variables)]
#![warn(
    deprecated_in_future,
    keyword_idents,
    trivial_casts,
    unused_import_braces,
)]
#[macro_use]
extern crate darling;
mod protobuf_model;
mod util;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ProtobufModel, attributes(protobuf_model))]
pub fn protobuf_model_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let st = parse_macro_input!(input as DeriveInput);
    protobuf_model::impl_protobuf_model_derive(&st).into()
}
