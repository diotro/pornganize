use protobuf::RepeatedField;
use serde::{Deserialize, Serialize};

use super::{
    Model,
    common::*,
    messages::{
        tag::Tag as TagMessage,
        common::{
            DateTime as MessageDateTime,
            ApplicableTo as MessageApplicableTo,
        }
    }
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub applicable_to: Vec<ApplicableTo>,
    pub also_adds: Vec<String>,
    pub aliases: Vec<String>,
    pub added_on: DateTime,
}

impl<'a> From<TagMessage> for &'a Tag {
    fn from(msg: TagMessage) -> Self {
        &&Tag {
            id: msg.id,
            name: msg.name,
            applicable_to: ProtobufEnumVec::from_vec(&msg.applicable_to).into(),
            also_adds: msg.also_adds.into_iter().collect(),
            aliases: msg.aliases.into_iter().collect(),
            added_on: match msg.added_on.as_ref() {
                Some(dt) => DateTime::from(dt),
                None => DateTime::now(),
            },
        }
    }
}

impl From<&Tag> for TagMessage {
    fn from(model: &Tag) -> Self {
        Self {
            id: model.id,
            name: model.name,
            applicable_to: to_vec(&model.applicable_to),
            also_adds: to_repeated_field(&model.also_adds),
            aliases: to_repeated_field(&model.aliases),
            added_on: (&model.added_on).into(),
            ..TagMessage::default()
        }
    }
}

pub struct TagModeler;

impl<'a> Modeler<'a> for TagModeler {
    type Model = Tag;
    type Message = TagMessage;
    const TREE_NAME: &'static str = "custom-fields";
    fn get_key(model: &'a Self::Model) -> &'a str { &model.id }
}
