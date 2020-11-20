use heck::KebabCase as _;
use darling::{ast, FromDeriveInput, util::Ignored, FromMeta};
use syn::{Ident, DeriveInput, Path, Type, NestedMeta, Lit};
use proc_macro2::{TokenStream, Span};

use super::util;

type Result<T> = std::result::Result<T, darling::Error>;

#[derive(Debug)]
enum Convert {
    NoConversion,
    Infer,
    Item(Path),
    Func(Path),
}

impl Default for Convert { fn default() -> Self { Convert::NoConversion } }

impl FromMeta for Convert {
    fn from_word() -> Result<Self> {
        Ok(Self::Infer)
    }

    fn from_string(value: &str) -> Result<Self> {
        Ok(Self::Item(util::str_to_path(value)))
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
        Ok(Self::Func(util::str_to_path(&name)))
    }
}

#[derive(Debug, FromMeta)]
#[darling(default)]
struct FieldNames {
    model: Option<String>,
    message: Option<String>,
}
impl Default for FieldNames { fn default() -> Self { Self { model: None, message: None } } }

#[derive(Debug, FromField)]
#[darling(attributes(protobuf_model))]
struct ProtobufModelField {
    ident: Option<Ident>,
    //ty: Type,
    #[darling(default)]
    key: bool,
    #[darling(default)]
    convert: Convert,
    #[darling(default)]
    names: FieldNames,
    #[darling(default)]
    skip: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(protobuf_model), supports(struct_named))]
struct ProtobufModelStruct {
    ident: Ident,
    #[darling(default)]
    tree: Option<String>,
    #[darling(default)]
    message: Option<Path>,
    data: ast::Data<Ignored, ProtobufModelField>,
}

fn get_key_field(fields: ast::Fields<ProtobufModelField>) -> Ident {
    let key_fields: Vec<Ident> = fields.into_iter()
        .filter(|x| { x.key })
        .map(|x| { x.ident.unwrap() })
        .collect();
    if key_fields.is_empty() { panic!("No key field provided!") }
    if key_fields.len() > 1 { panic!("Multiple key fields provided!") }
    key_fields[0].clone()
}

#[allow(clippy::or_fun_call)]
pub fn impl_protobuf_model_derive(parsed: &DeriveInput) -> TokenStream {
    let protobuf_model = ProtobufModelStruct::from_derive_input(parsed).unwrap();
    let fields = if let ast::Data::Struct(data) = protobuf_model.data {
        data
    } else {
        panic!("ProtobufModel structs must contain fields!");
    };
    dbg!(fields);
    panic!("");
    let key_field = get_key_field(fields);
    let message = protobuf_model.message.unwrap_or(
        Path::from(quote::format_ident!("{}Message", protobuf_model.ident))
    );
    let tree = protobuf_model.tree.unwrap_or(format!("{}", protobuf_model.ident).to_kebab_case());
    let struct_ident = protobuf_model.ident;
    quote::quote! {
        impl crate::model::Model for #struct_ident {
            const TREE_NAME: &'static str = #tree;
            fn get_key(&self) -> &str { &self.#key_field }
            fn to_bytes(self) -> Vec<u8> {
                #message::from(self).write_to_bytes().unwrap()
            }

            fn from_bytes(bytes: &[u8]) -> Self {
                ::protobuf::parse_from_bytes::<#message>(bytes).unwrap().into()
            }
        }
    }
}
