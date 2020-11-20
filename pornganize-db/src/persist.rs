use std::collections::HashMap;

use sled::{
    Db as SledDb,
    Tree,
    IVec,
    Config as SledConfig,
};

use pornganize::config::{Config, DatabaseCfg};
use crate::model::{Model, Modeler};

const DATABASE_VERSION_KEY: &str = "__database_version__";
pub const CURRENT_DATABASE_VERSION: &str = env!("PORNGANIZE_DB_VERSION");

#[derive(Debug)]
pub enum Error {
    DatabaseVersionError{db_version: String, current_version: String},
    SledError(sled::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Database {
    sled_db: SledDb,
    trees: HashMap<&'static str, Tree>,
    version: IVec,
}

impl Database {
    pub fn new(cfg: &DatabaseCfg) -> Result<Self>{
        let db = Self {
            sled_db: SledConfig::new()
                .path(cfg.path.to_owned())
                .use_compression(true)
                .open().unwrap(),
            trees: HashMap::new(),
            version: CURRENT_DATABASE_VERSION.into(),
        };
        db.check_version()?;
        db.sled_db.insert(DATABASE_VERSION_KEY, CURRENT_DATABASE_VERSION).unwrap();
        Ok(db)
    }

    fn check_version(&self) -> Result<()> {
        match self.sled_db.get(DATABASE_VERSION_KEY).unwrap() {
            Some(version) =>
                if version == self.version {
                    Ok(())
                } else {
                    Err(Error::DatabaseVersionError{
                        db_version: String::from_utf8((*version).into()).unwrap(),
                        current_version: String::from(CURRENT_DATABASE_VERSION)
                    })
                },
            None => Ok(())
        }
    }

    fn get_tree(&mut self, tree_name: &'static str) -> &Tree {
        if !self.trees.contains_key(tree_name) {
            let tree = self.sled_db.open_tree(tree_name).unwrap();
            self.trees.insert(tree_name, tree);
        }
        self.trees.get(tree_name).unwrap()
    }

    pub fn store<M: Model>(&mut self, model: M) -> Result<()>{
        let key = String::from(model.get_key());
        let data = model.to_bytes();
        self.get_tree(<M as Model>::TREE_NAME).insert(key, data).unwrap();
        Ok(())
    }

    pub fn get<M: Model>(&mut self, key: &str) -> Result<Option<M>> {
        match self.get_tree(<M as Model>::TREE_NAME).get(key).unwrap() {
            Some(data) => Ok(Some(M::from_bytes(data.as_ref()))),
            None => Ok(None),
        }
    }
}
