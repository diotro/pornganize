use serde::{Deserialize, Serialize};
use super::{
    messages::tag::Tag as TagMessage,
    DateTime,
    to_repeated_field,
    from_repeated_field,
    convert_vec,
    ApplicableTo
};

#[derive(Debug, Serialize, Deserialize, ProtobufModel)]
pub struct Tag {
    #[protobuf_model(key)]
    pub id: String,
    pub name: String,
    #[protobuf_model(
        msg2model("convert_vec"),
        model2msg("convert_vec"),
    )]
    pub applicable_to: Vec<ApplicableTo>,
    #[protobuf_model(
        msg2model("from_repeated_field"),
        model2msg("to_repeated_field"),
    )]
    pub also_adds: Vec<String>,
    #[protobuf_model(
        msg2model("from_repeated_field"),
        model2msg("to_repeated_field"),
    )]
    pub aliases: Vec<String>,
    #[protobuf_model(
        msg2model("DateTime::or_now"),
        model2msg="infer",
    )]
    pub added_on: DateTime,
}
