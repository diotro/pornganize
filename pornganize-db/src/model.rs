
pub trait Model {
    const TREE_NAME: &'static str;
    fn get_key(&self) -> &str;
    fn to_bytes(self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Self;
}

mod common;
pub (crate) use common::*;
mod messages;

//pub mod actor;
//pub mod custom;
//pub mod dvd;
//pub mod game;
//pub mod manga;
//pub mod network;
pub mod site;
//pub mod studio;
//pub mod tag;
pub mod sandbox;
//pub mod video;
