use std::collections::HashMap;
use tantivy::{
    self,
    schema::{Schema, Field, IntOptions, TextOptions, Value},
    Document
};
use serde::{Deserialize, Serialize};


pub enum FieldType {
    Bytes,
    Facet,
    Date(IntOptions),
    F64(IntOptions),
    I64(IntOptions),
    U64(IntOptions),
    Text(TextOptions),
}

pub struct ModelField<'a>(&'a str, FieldType);

pub struct SearchSchema {
    name: String,
    schema: Schema,
    fields: HashMap<String, Field>,
}
impl SearchSchema {
    pub fn new(name: &str, fields: &Vec<ModelField<'_>>) -> Self {
        let builder = Schema::builder();
        let field_map: HashMap<String, Field> = HashMap::new();
        for model_field in fields {
            let field = match model_field.1 {
                FieldType::Bytes => builder.add_bytes_field(model_field.0),
                FieldType::Facet => builder.add_facet_field(model_field.0),
                FieldType::Date(opts) => builder.add_date_field(model_field.0, opts),
                FieldType::F64(opts) => builder.add_f64_field(model_field.0, opts),
                FieldType::I64(opts) => builder.add_i64_field(model_field.0, opts),
                FieldType::U64(opts) => builder.add_u64_field(model_field.0, opts),
                FieldType::Text(opts) => builder.add_text_field(model_field.0, opts),
            };
            field_map.insert(model_field.0.to_string(), field);
        }
        Self {
            name: name.to_string(),
            schema: builder.build(),
            fields: field_map,
        }
    }

    pub fn get_field<T>(&self, name: &str) -> Option<&Field> {
        self.fields.get(name)
    }

    fn get_value(&self, doc: Document, field: &str) -> Option<&Value> {
        let field = match self.get_field(field) {
            None => return None,
            Some(f) => f,
        }
        let val = doc.get_first(field);
    }

    //pub fn get_f64_value(&self, doc: Document, field: &str) -> Option<f64> {
        //let field = self.get_field(field);
    //}
}

//TODO add Deserialize (lifetime mod is getting in the way).
pub trait Modelable {
}

pub mod actors;
pub mod dvds;
pub mod games;
pub mod mangas;
pub mod sites;
pub mod studios;
pub mod tags;
pub mod videos;
