use protobuf::Message;

pub trait Model<M>: From<M> + Into<M>
where
    M: Message,
{
    const TREE_NAME: &'static str;
    fn get_key<'a>(&'a self) -> &'a str;
}

mod common;
pub (crate) use common::*;
mod messages;

pub mod actor;
pub mod dvd;
pub mod game;
pub mod manga;
pub mod site;
pub mod studio;
pub mod tag;
pub mod video;
