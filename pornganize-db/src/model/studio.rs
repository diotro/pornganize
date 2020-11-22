use serde::{Deserialize, Serialize};
use protobuf::SingularPtrField;
use super::{
    DateTime,
    ptr_to_option,
    option_to_ptr,
    messages::studio::Studio as StudioMessage,
};
use pornganize_macros::ProtobufModel;

#[derive(Debug, Serialize, Deserialize, ProtobufModel)]
pub struct Studio {
    #[protobuf_model(key)]
    pub id: String,
    pub name: String,
    pub banner: String,
    pub logo: String,
    pub website: String,
    pub description: String,
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
