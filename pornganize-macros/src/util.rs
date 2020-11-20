use syn::{Ident, Path};
use proc_macro2::Span;


pub fn str_to_path(value: &str) -> Path {
    Path::from(Ident::new(value, Span::call_site()))
}
