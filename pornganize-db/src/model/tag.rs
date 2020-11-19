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

impl From<TagMessage> for Tag {
    fn from(msg: TagMessage) -> Self {
        let added_on: DateTime = match msg.added_on.into_option() {
            Some(dt) => DateTime::from(dt),
            None => DateTime::now(),
        };
        Self {
            id: msg.id,
            name: msg.name,
            applicable_to: ProtobufEnumVec::from_vec(msg.applicable_to).into(),
            also_adds: msg.also_adds.into_vec(),
            aliases: msg.aliases.into_vec(),
            added_on,
        }
    }
}

impl Model<TagMessage> for Tag {
    fn tree_name() -> &'static str {
        "tags"
    }

    fn get_key(&self) -> String {
        self.id.clone()
    }
}

impl From<Tag> for TagMessage {
    fn from(model: Tag) -> Self {
        Self {
            id: model.id,
            name: model.name,
            applicable_to: to_vec(model.applicable_to),
            also_adds: to_repeated_field(model.also_adds),
            aliases: to_repeated_field(model.aliases),
            added_on: model.added_on.into(),
            ..TagMessage::default()
        }
    }
}
