use anyhow::Result;
use crate::{
    db::{
        ItemType,
        Database
    },
    config::Config,
    plugins::Plugin,
};

pub trait Runner {
    fn new(config: &Config) -> Self;
    fn db(&self) -> &Database;
    fn db_mut(&mut self) -> &mut Database;
    fn register_type(&mut self, item_type: ItemType) -> Result<()>;
    fn get_type(&self, type_id: &str) -> ItemType;
    fn register_plugin(&mut self, plugin: impl Plugin) -> Result<()>;
}
