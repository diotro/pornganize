use serde::{Deserialize, Serialize};
use super::{
    DateTime,
    messages::actor::Actor as ActorMessage,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub profile1: Option<String>,
    pub profile2: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareerStatus {
    pub started: Option<DateTime>,
    pub is_active: bool,
    pub retired: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, ProtobufModel)]
pub struct Actor {
    #[protobuf_model(key)]
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub name_patterns: Vec<String>,
    //gender: 'static &Gender,
    pub nationality: String,
    pub dob: String,
    #[protobuf_model(
        msg2model("DateTime::or_now"),
        model2msg="infer",
    )]
    pub added_on: DateTime,
}
