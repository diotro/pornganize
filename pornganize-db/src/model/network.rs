use serde::{Deserialize, Serialize};
use protobuf::SingularPtrField;
use super::{
    Model,
    DateTime,
    messages::network::Network as NetworkMessage,
};


#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    id: String,
    name: String,
    banner: String,
    logo: String,
    website: String,
    description: String,
    studio_id: Option<String>,
    established: Option<DateTime>,
    added_on: DateTime,
}

impl<'a> From<NetworkMessage> for &'a Network {
    fn from(msg: NetworkMessage) -> Self {
        &&Network {
            id: msg.id.clone(),
            name: msg.name.clone(),
            banner: msg.banner.clone(),
            logo: msg.logo.clone(),
            website: msg.website.clone(),
            description: msg.description.clone(),
            studio_id:
                if msg.studio_id.is_empty() { None }
                else { Some(msg.studio_id.clone()) },
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

impl From<&Network> for NetworkMessage {
    fn from(network: &Network) -> Self {
        Self {
            id: network.id.clone(),
            name: network.name.clone(),
            banner: network.banner.clone(),
            logo: network.logo.clone(),
            website: network.website.clone(),
            description: network.description.clone(),
            studio_id: String::from(network.studio_id.as_deref().unwrap_or("")),
            established: SingularPtrField::from_option(match network.established.as_ref() {
                Some(dt) => Some(dt.into()),
                None => None
            }),
            added_on: (&network.added_on).into(),
            .. Self::default()
        }
    }
}

pub struct NetworkModeler;

impl<'a> Modeler<'a> for NetworkModeler {
    type Model = Network;
    type Message = NetworkMessage;
    const TREE_NAME: &'static str = "custom-fields";
    fn get_key(model: &'a Self::Model) -> &'a str { &model.id }
}
