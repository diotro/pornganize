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

impl From<StudioMessage> for Studio {
    fn from(msg: StudioMessage) -> Self {
        let added_on: DateTime = match msg.added_on.into_option() {
            Some(dt) => DateTime::from(dt),
            None => DateTime::now(),
        };
        Self {
            id: msg.id,
            name: msg.name,
            banner: msg.banner,
            logo: msg.logo,
            website: msg.website,
            description: msg.description,
            established: match msg.established.into_option() {
                Some(dt) => Some(DateTime::from(dt)),
                None => None,
            },
            added_on,
        }
    }
}

impl Model<StudioMessage> for Studio {
    fn tree_name() -> &'static str { "studios" }
    fn get_key(&self) -> String { self.id.clone() }
}

impl From<Studio> for StudioMessage {
    fn from(studio: Studio) -> Self {
        Self {
            id: studio.id,
            name: studio.name,
            banner: studio.banner,
            logo: studio.logo,
            website: studio.website,
            description: studio.description,
            established: SingularPtrField::from_option(match studio.established {
                Some(dt) => Some(dt.into()),
                None => None
            }),
            added_on: studio.added_on.into(),
            .. Self::default()
        }
    }
}
