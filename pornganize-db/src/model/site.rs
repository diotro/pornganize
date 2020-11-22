//use serde::{Deserialize, Serialize};
use super::{
    DateTime,
    ptr_to_option,
    option_to_ptr,
    messages::site::Site as SiteMessage,
};

//#[derive(Debug, Serialize, Deserialize, ProtobufModel)]
#[derive(ProtobufModel)]
pub struct Site {
    #[protobuf_model(key)]
    pub id: String,
    pub name: String,
    pub banner: String,
    pub logo: String,
    pub url: String,
    pub description: String,
    pub studio_id: Option<String>,
    pub network_id: Option<String>,
    #[protobuf_model(
        msg2model("ptr_to_option"),
        model2msg("option_to_ptr"),
    )]
    pub established: Option<DateTime>,
    #[protobuf_model(
        msg2model("DateTime::or_now"),
        model2msg="infer",
    )]
    pub added_on: DateTime,
}
