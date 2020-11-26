use std::collections::HashMap;
use anyhow::Result;
use sled::{
    Db,
    Tree,
    IVec,
};
use tantivy::{
    schema::Schema,
    IndexWriter,
    Index,
};
use crate::config::DatabaseCfg;

pub use json::object::Object as JsonObject;

/// Each type object has a property that defines what type it is. This is the name of that property.
pub const TYPE_ID_PROPERTY: &str = "~~type~id~~";

pub trait IndexHandler {}

pub struct ItemType
{
    type_id: String,
    search_schema: Schema,
}

impl ItemType
{
    pub fn new<H: IndexHandler>(type_id: &str, search_schema: Schema) -> Self {
        Self {
            type_id: String::from(type_id),
            search_schema,
        }
    }
    pub fn type_id(&self) -> &str { &self.type_id }
    pub fn search_schema(&self) -> Option<&Schema> { Some(&self.search_schema) }
}

struct ItemTypeDbMetadata {
    pub tree: Tree,
    pub index: Index,
}

pub struct Database {
    sled_db: Db,
    type_registry: HashMap<String, ItemTypeDbMetadata>,
}

impl Database {
    pub fn new(db_cfg: &DatabaseCfg) -> Result<Self> {
        Ok(Self {
            sled_db: sled::open(db_cfg.path.as_path())?,
            type_registry: HashMap::new(),
        })
    }

    pub fn get_item(&mut self, key: &str, type_id: &str) -> Result<Option<JsonObject>> {
        todo!()
    }

    pub fn save_item(&mut self, item: &JsonObject) -> Result<()> {
        todo!()
    }
}
