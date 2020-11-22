use serde::{Deserialize, Serialize};
use super::{
    DateTime,
    ptr_to_option,
    option_to_ptr,
    messages::video::Video as VideoMessage,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Marker {
    pub id: u32,
    pub label: String,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resolution {
    pub height: u16,
    pub width: u16,
}

//#[derive(Debug, Serialize, Deserialize, ProtobufModel)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    //#[protobuf_model(key)]
    pub id: String,
    pub path: String,
    pub title: String,
    pub resolution: Resolution,
    pub vcodec: String,
    pub acodec: String,
    pub duration: u64,
    pub framerate: i16,
    pub markers: Vec<Marker>,
    pub actors: Vec<String>,
    pub thumbnails: Vec<String>,
    pub tags: Vec<String>,
    //#[protobuf_model(
        //msg2model("DateTime::or_now"),
        //model2msg="infer",
    //)]
    pub added_on: DateTime,
}
