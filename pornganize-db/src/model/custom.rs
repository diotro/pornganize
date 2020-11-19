use serde::{Deserialize, Serialize};
use protobuf::SingularPtrField;
use super::{
    Model,
    DateTime,
    messages::site::CustomField as CustomFieldMessage,
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
        let added_on: DateTime = match msg.added_on.into_option() {
            Some(dt) => DateTime::from(dt),
            None => DateTime::now(),
        };
        Self {
            id: msg.id,
            name: msg.name,
            applicable_to: ProtobufEnumVec::from_vec(msg.applicable_to).into(),
            description: msg.description,
            added_on,
        }
    }
}

impl Model<CustomFieldMessage> for CustomField {
    fn tree_name() -> &'static str { "custom-fields" }
    fn get_key(&self) -> String { self.id.clone() }
}

impl From<CustomField> for CustomFieldMessage {
    fn from(model: CustomField) -> Self {
        Self {
            id: model.id,
            name: model.name,
            applicable_to: to_vec(model.applicable_to),
            description: model.description,
            added_on: model.added_on.into(),
            ..CustomFieldMessage::default()
        }
    }
}
