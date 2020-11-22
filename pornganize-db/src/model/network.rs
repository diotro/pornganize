use serde::{Deserialize, Serialize};
use protobuf::SingularPtrField;
use super::{
    DateTime,
    ptr_to_option,
    option_to_ptr,
    messages::network::Network as NetworkMessage,
};


#[derive(Debug, Serialize, Deserialize, ProtobufModel)]
pub struct Network {
    #[protobuf_model(key)]
    pub id: String,
    pub name: String,
    pub banner: String,
    pub logo: String,
    pub website: String,
    pub description: String,
    pub studio_id: Option<String>,
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
