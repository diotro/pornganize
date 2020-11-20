pub use std::ops::Deref;
use protobuf::{Message, parse_from_bytes};


pub trait Modeler<'a, M>
{
    type Model: 'a + From<Self::Message>;
    type Message: Message + From<Self::Model>;
    const TREE_NAME: &'static str;

    fn get_key(model: &'a Self::Model) -> &'a str;

    fn to_bytes(model: Self::Model) -> Vec<u8> {
        Self::Message::from(model).write_to_bytes().unwrap()
    }

    fn from_bytes(bytes: &'a [u8]) -> Self::Model {
        parse_from_bytes::<Self::Message >(bytes).unwrap().into()
    }
}

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
pub mod custom;
//pub mod dvd;
//pub mod game;
//pub mod manga;
//pub mod network;
//pub mod site;
//pub mod studio;
//pub mod tag;
//pub mod video;
