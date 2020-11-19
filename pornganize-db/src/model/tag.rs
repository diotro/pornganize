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

impl From<&TagMessage> for Tag {
    fn from(msg: &TagMessage) -> Self {
        Self {
            id: msg.id.clone(),
            name: msg.name.clone(),
            applicable_to: ProtobufEnumVec::from_vec(&msg.applicable_to).into(),
            also_adds: msg.also_adds.iter().map(|x| {x.clone()}).collect(),
            aliases: msg.aliases.iter().map(|x| {x.clone()}).collect(),
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
            id: model.id.clone(),
            name: model.name.clone(),
            applicable_to: to_vec(&model.applicable_to),
            also_adds: to_repeated_field(&model.also_adds),
            aliases: to_repeated_field(&model.aliases),
            added_on: (&model.added_on).into(),
            ..TagMessage::default()
        }
    }
}

impl Model<'_, TagMessage> for Tag {
    const TREE_NAME: &'static str = "tags";
    fn get_key(&self) -> &str { &self.id }
}
