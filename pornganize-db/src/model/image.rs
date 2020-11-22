use serde::{Deserialize, Serialize};
use super::{
    DateTime,
    messages::image::Image as ImageMessage,
};

#[derive(Debug, Serialize, Deserialize, ProtobufModel)]
pub struct Image {
    #[protobuf_model(key)]
    pub id: String,
    pub path: String,
    #[protobuf_model(
        msg2model("DateTime::or_now"),
        model2msg="infer",
    )]
    pub added_on: DateTime,
}
