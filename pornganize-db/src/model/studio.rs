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

impl<'a> From<StudioMessage> for &'a Studio {
    fn from(msg: StudioMessage) -> Self {
        &&Studio {
            id: msg.id,
            name: msg.name,
            banner: msg.banner,
            logo: msg.logo,
            website: msg.website,
            description: msg.description,
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
            id: studio.id,
            name: studio.name,
            banner: studio.banner,
            logo: studio.logo,
            website: studio.website,
            description: studio.description,
            established: SingularPtrField::from_option(match studio.established.as_ref() {
                Some(dt) => Some(dt.into()),
                None => None
            }),
            added_on: (&studio.added_on).into(),
            .. Self::default()
        }
    }
}


pub struct StudioModeler;

impl<'a> Modeler<'a> for StudioModeler {
    type Model = Studio;
    type Message = StudioMessage;
    const TREE_NAME: &'static str = "studios";
    fn get_key(model: &'a Self::Model) -> &'a str { &model.id }
}
