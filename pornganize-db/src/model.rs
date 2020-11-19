use protobuf::Message;

pub trait Model<'a, M>
where
    Self: 'a + From<&'a M>,
    M: Message + From<&'a Self>,
{
    const TREE_NAME: &'static str;
    fn get_key(&self) -> &str;
    fn to_bytes(&'a self) -> Vec<u8> {
        M::from(&self).write_to_bytes().unwrap()
    }
}

mod common;
pub (crate) use common::*;
mod messages;

pub mod actor;
pub mod custom;
pub mod dvd;
pub mod game;
pub mod manga;
pub mod network;
pub mod site;
pub mod studio;
pub mod tag;
pub mod video;
