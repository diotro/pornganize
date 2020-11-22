use serde::{Deserialize, Serialize};
use super::{
    DateTime,
    messages::dvd::Dvd as DvdMessage,
};

#[derive(Debug, Serialize, Deserialize, ProtobufModel)]
pub struct Dvd {
    #[protobuf_model(key)]
    pub id: String,
    pub title: String,
    pub cover_img: String,
    pub back_img: String,
    #[protobuf_model(
        msg2model("DateTime::or_now"),
        model2msg="infer",
    )]
    pub added_on: DateTime,
}
