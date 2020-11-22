use serde::{Deserialize, Serialize};
use super::{
    DateTime,
    messages::manga::Manga as MangaMessage,
};

#[derive(Debug, Serialize, Deserialize, ProtobufModel)]
pub struct Manga {
    #[protobuf_model(key)]
    pub id: String,
    #[protobuf_model(
        msg2model("DateTime::or_now"),
        model2msg="infer",
    )]
    pub added_on: DateTime,
}
