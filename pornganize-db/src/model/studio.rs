use serde::{Deserialize, Serialize};
use protobuf::SingularPtrField;
use super::{
    Model,
    DateTime,
    messages::studio::Studio as StudioMessage,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Studio {
    id: String,
    name: String,
    banner: String,
    logo: String,
    website: String,
    description: String,
    established: Option<DateTime>,
    added_on: DateTime,
}

impl From<&StudioMessage> for Studio {
    fn from(msg: &StudioMessage) -> Self {
        Self {
            id: msg.id.clone(),
            name: msg.name.clone(),
            banner: msg.banner.clone(),
            logo: msg.logo.clone(),
            website: msg.website.clone(),
            description: msg.description.clone(),
            established: match msg.established.as_ref() {
                Some(dt) => Some(DateTime::from(dt)),
                None => None,
            },
            added_on: match msg.added_on.as_ref() {
                Some(dt) => DateTime::from(dt),
                None => DateTime::now(),
            },
        }
    }
}

impl From<&Studio> for StudioMessage {
    fn from(studio: &Studio) -> Self {
        Self {
            id: studio.id.clone(),
            name: studio.name.clone(),
            banner: studio.banner.clone(),
            logo: studio.logo.clone(),
            website: studio.website.clone(),
            description: studio.description.clone(),
            established: SingularPtrField::from_option(match studio.established.as_ref() {
                Some(dt) => Some(dt.into()),
                None => None
            }),
            added_on: (&studio.added_on).into(),
            .. Self::default()
        }
    }
}

impl Model<'_, StudioMessage> for Studio {
    const TREE_NAME: &'static str = "studios";
    fn get_key(&self) -> &str { &self.id }
}
