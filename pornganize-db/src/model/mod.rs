use protobuf::Message;

pub trait Model<M>: From<M> + Into<M>
where
    M: Message,
{
    fn tree_name() -> &'static str;
    fn get_key(&self) -> String;
}

mod common;
pub use common::*;
mod messages;

pub mod actor;
pub mod dvd;
pub mod game;
pub mod manga;
pub mod site;
pub mod studio;
pub mod tag;
pub mod video;
