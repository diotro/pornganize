use crate::config::SectionCfg;
use crate::db::ItemType;

pub struct Section {
    id: String,
    title: String,
    config: SectionCfg,
}
