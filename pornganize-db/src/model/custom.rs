use serde::{Deserialize, Serialize};
use protobuf::SingularPtrField;
use super::{
    Model,
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

impl From<&CustomFieldMessage> for CustomField {
    fn from(msg: &CustomFieldMessage) -> Self {
        Self {
            id: msg.id.clone(),
            name: msg.name.clone(),
            applicable_to: ProtobufEnumVec::from_vec(&msg.applicable_to).into(),
            description: msg.description.clone(),
            added_on: match msg.added_on.as_ref() {
                Some(dt) => DateTime::from(dt),
                None => DateTime::now(),
            },
        }
    }
}

impl From<&CustomField> for CustomFieldMessage {
    fn from(model: &CustomField) -> Self {
        Self {
            id: model.id.clone(),
            name: model.name.clone(),
            applicable_to: to_vec(&model.applicable_to),
            description: model.description.clone(),
            added_on: (&model.added_on).into(),
            ..CustomFieldMessage::default()
        }
    }
}

impl Model<'_, CustomFieldMessage> for CustomField {
    const TREE_NAME: &'static str = "custom-fields";
    fn get_key(&self) -> &str { &self.id }
}
