use serde::{Deserialize, Serialize};
use super::{
    DateTime,
    messages::game::Game as GameMessage,
};

#[derive(Debug, Serialize, Deserialize, ProtobufModel)]
pub struct Game {
    #[protobuf_model(key)]
    pub id: String,
    pub name: String,
    #[protobuf_model(
        msg2model("DateTime::or_now"),
        model2msg="infer",
    )]
    pub added_on: DateTime,
}
