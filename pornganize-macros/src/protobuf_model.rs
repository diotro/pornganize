use anyhow::bail;
use darling::{ast, util::Ignored, FromDeriveInput, FromMeta};
use heck::KebabCase as _;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::env;
use syn::{DeriveInput, Ident, Lit, NestedMeta, Path, Type};

use super::util;

type Result<T> = std::result::Result<T, darling::Error>;

#[derive(Debug)]
enum Convert {
    NoConversion,
    Infer,
    Item(Path),
    Func(Path),
}

impl Default for Convert {
    fn default() -> Self {
        Convert::NoConversion
    }
}

impl FromMeta for Convert {
    fn from_string(value: &str) -> Result<Self> {
        if value == "infer" {
            Ok(Self::Infer)
        } else {
            Ok(Self::Item(syn::parse_str(value).unwrap()))
        }
    }

    fn from_list(items: &[NestedMeta]) -> Result<Self> {
        if items.is_empty() {
            todo!()
        }
        if items.len() > 1 {
            todo!()
        }
        let name = match &items[0] {
            NestedMeta::Lit(lit) => match lit {
                Lit::Str(s) => s.value(),
                _ => todo!(),
            },
            _ => todo!(),
        };
        Ok(Self::Func(syn::parse_str(&name).unwrap()))
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(protobuf_model))]
struct ProtobufModelField {
    ident: Option<Ident>,
    //ty: Type,
    #[darling(default)]
    key: bool,
    #[darling(default, rename = "msg2model")]
    to_model: Convert,
    #[darling(default, rename = "model2msg")]
    to_msg: Convert,
    #[darling(default)]
    rename: Option<Ident>,
    #[darling(default)]
    skip: bool,
}

#[allow(clippy::or_fun_call)]
impl ProtobufModelField {
    fn get_names(&self) -> (Ident, Ident) {
        let model_name = self.ident.clone().unwrap();
        let msg_name = self.rename.clone().unwrap_or(model_name.clone());
        (model_name, msg_name)
    }

    pub fn model2msg(&self, model_ident: &Ident) -> Result<TokenStream> {
        let (model_name, msg_name) = self.get_names();
        let lhs = match &self.to_msg {
            Convert::NoConversion => quote! { #model_ident.#msg_name },
            Convert::Infer => quote! { #model_ident.#msg_name.into() },
            Convert::Item(p) => quote! { #p::from(#model_ident.#msg_name) },
            Convert::Func(p) => quote! { #p(#model_ident.#msg_name) },
        };
        Ok(quote! { #model_name: #lhs,})
    }

    pub fn msg2model(&self, msg_ident: &Ident) -> Result<TokenStream> {
        let (model_name, msg_name) = self.get_names();
        let lhs = match &self.to_model {
            Convert::NoConversion => quote! { #msg_ident.#model_name },
            Convert::Infer => quote! { #msg_ident.#model_name.into() },
            Convert::Item(p) => quote! { #p::from(#msg_ident.#model_name) },
            Convert::Func(p) => quote! { #p(#msg_ident.#model_name) },
        };
        Ok(quote! { #msg_name: #lhs,})
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(protobuf_model), supports(struct_named))]
struct ProtobufModelStruct {
    ident: Ident,
    #[darling(default)]
    tree: Option<String>,
    #[darling(default)]
    message: Option<Path>,
    #[darling(default)]
    defaults: bool,
    data: ast::Data<Ignored, ProtobufModelField>,
}

fn get_key_field(fields: ast::Fields<ProtobufModelField>) -> Ident {
    let key_fields: Vec<Ident> = fields
        .into_iter()
        .filter(|x| x.key)
        .map(|x| x.ident.unwrap())
        .collect();
    if key_fields.is_empty() {
        panic!("No key field provided!")
    }
    if key_fields.len() > 1 {
        panic!("Multiple key fields provided!")
    }
    key_fields[0].clone()
}

fn process_field(
    field: ProtobufModelField,
    model_name: &Ident,
    msg_name: &Ident,
    model2msg: &mut TokenStream,
    msg2model: &mut TokenStream,
) -> Option<Ident> {
    model2msg.extend(field.model2msg(model_name));
    msg2model.extend(field.msg2model(msg_name));
    if field.key {
        field.ident
    } else {
        None
    }
}

struct ProcessFieldsResponse {
    key: Ident,
    model2msg: TokenStream,
    msg2model: TokenStream,
}

fn process_fields(
    fields: ast::Fields<ProtobufModelField>,
    model_name: &Ident,
    msg_name: &Ident,
) -> Result<ProcessFieldsResponse> {
    let mut model2msg = TokenStream::new();
    let mut msg2model = TokenStream::new();
    let key_fields: Vec<Ident> = fields
        .into_iter()
        .filter_map(|x| process_field(x, model_name, msg_name, &mut model2msg, &mut msg2model))
        .collect();
    if key_fields.is_empty() {
        panic!("No key field provided!")
    }
    if key_fields.len() > 1 {
        panic!("Multiple key fields provided!")
    }
    Ok(ProcessFieldsResponse {
        model2msg,
        msg2model,
        key: key_fields[0].clone(),
    })
}

#[allow(clippy::or_fun_call)]
fn get_model_trait_path() -> TokenStream {
    let crate_name = env::var("CARGO_CRATE_NAME").unwrap_or(String::from(""));
    if crate_name == "pornganize_db" || crate_name == "pornganize-db" {
        quote! { crate::model::Model }
    } else {
        quote! { ::pornganize_db::model::Model }
    }
}

#[allow(clippy::or_fun_call)]
pub fn impl_protobuf_model_derive(parsed: &DeriveInput) -> TokenStream {
    let protobuf_model = ProtobufModelStruct::from_derive_input(parsed).unwrap();
    let fields = if let ast::Data::Struct(data) = protobuf_model.data {
        data
    } else {
        panic!("Model structs must contain fields!");
    };
    let model_name = format_ident!("model");
    let msg_name = format_ident!("msg");
    let ProcessFieldsResponse {
        key,
        model2msg,
        msg2model,
    } = process_fields(fields, &model_name, &msg_name).unwrap();
    let message = protobuf_model
        .message
        .unwrap_or(Path::from(format_ident!("{}Message", protobuf_model.ident)));
    let tree = protobuf_model
        .tree
        .unwrap_or(format!("{}", protobuf_model.ident).to_kebab_case());
    let model = protobuf_model.ident;
    let model_trait_path = get_model_trait_path();
    quote! {
        use ::protobuf::Message as _;
        impl From<#message> for #model {
            fn from(#msg_name: #message) -> Self {
                Self {
                    #msg2model
                }
            }
        }
        impl From<#model> for #message {
            fn from(#model_name: #model) -> Self {
                Self {
                    #model2msg
                    ..#message::default()
                }
            }
        }
        impl #model_trait_path for #model {
            const TREE_NAME: &'static str = #tree;
            fn get_key(&self) -> &str { &self.#key }
            fn to_bytes(self) -> Vec<u8> {
                #message::from(self).write_to_bytes().unwrap()
            }

            fn from_bytes(bytes: &[u8]) -> Self {
                ::protobuf::parse_from_bytes::<#message>(bytes).unwrap().into()
            }
        }
    }
}
