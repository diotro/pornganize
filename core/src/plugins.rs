use anyhow::{
    Error,
    Result,
};
use async_trait::async_trait;
use crate::{
    config::{
        Config,
        PluginCfg,
    },
    db::{
        ItemType,
        JsonObject,
    },
};

/// A plugin that operates on a database item. This gets before an item is added, updated or
/// removed.
#[async_trait]
pub trait Plugin: Default
{
    fn init(&mut self, global_cfg: &Config, plugin_cfg: &PluginCfg) -> Result<()>;

    fn plugin_id(&self) -> &str;

    fn get_item_types(&self) -> &[&str];

    async fn on_created<'a>(& self, item: &'a JsonObject) -> &'a JsonObject;

    async fn on_updated<'a>(&self, item: &'a JsonObject) -> &'a JsonObject;

    #[allow(unused_variables)]
    async fn on_deleted(&self, item: &JsonObject);

    #[allow(unused_variables)]
    async fn on_error<'a>(&self, item: &JsonObject, error: &'a Error) -> Option<&'a Error>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
