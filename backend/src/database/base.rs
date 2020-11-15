use crate::config::DatabaseCfg

pub trait DatabaseManager<B: DatabaseCfg>{
    fn new(config: B) -> Self;

    /// Sets up the data base for the first time.
    fn setup(&self) -> Result<(), _>;

    /// Initialize the manager.
    fn init(&self) -> Result<(), _>;
}
