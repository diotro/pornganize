use serde::{Deserialize, Serialize};
use protobuf::{SingularPtrField, parse_from_bytes, Message};
use super::{
    Modeler, Model,
    common::{
        DateTime,
        ApplicableTo,
        ProtobufEnumVec,
        to_vec,
    },
    messages::custom::CustomField as CustomFieldMessage,
};


#[derive(Debug, Serialize, Deserialize)]
pub struct CustomField {
    pub id: String,
    pub name: String,
    pub applicable_to: Vec<ApplicableTo>,
    pub description: String,
    pub added_on: DateTime,
}

impl From<CustomFieldMessage> for CustomField {
    fn from(msg: CustomFieldMessage) -> Self {
        CustomField {
            id: msg.id,
            name: msg.name,
            applicable_to: ProtobufEnumVec::from_vec(&msg.applicable_to).into(),
            description: msg.description,
            added_on: match msg.added_on.as_ref() {
                Some(dt) => DateTime::from(dt),
                None => DateTime::now(),
            },
        }
    }
}

impl From<CustomField> for CustomFieldMessage {
    fn from(model: CustomField) -> Self {
        Self {
            id: model.id,
            name: model.name,
            applicable_to: to_vec(&model.applicable_to),
            description: model.description,
            added_on: (&model.added_on).into(),
            ..CustomFieldMessage::default()
        }
    }
}

impl Model for CustomField {
    const TREE_NAME: &'static str = "custom-fields";
    fn get_key(&self) -> &str { &self.id }
    fn to_bytes(self) -> Vec<u8> {
        CustomFieldMessage::from(self).write_to_bytes().unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        parse_from_bytes::<CustomFieldMessage>(bytes).unwrap().into()
    }
}
