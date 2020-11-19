use serde::{Deserialize, Serialize};
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

impl From<NetworkMessage> for Network {
    fn from(msg: NetworkMessage) -> Self {
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
            studio_id: if msg.studio_id.is_empty() { None } else { Some(msg.studio_id) },
            established: match msg.established.into_option() {
                Some(dt) => Some(DateTime::from(dt)),
                None => None,
            },
            added_on,
        }
    }
}

impl Model<NetworkMessage> for Network {
    fn tree_name() -> &'static str { "networks" }

    fn get_key(&self) -> String {
        self.id.clone()
    }
}

impl From<Network> for NetworkMessage {
    fn from(network: Network) -> Self {
        Self {
            id: network.id,
            name: network.name,
            banner: network.banner,
            logo: network.logo,
            website: network.website,
            description: network.description,
            studio_id: network.studio_id.unwrap_or(String::from("")),
            established: SingularPtrField::from_option(match network.established {
                Some(dt) => Some(dt.into()),
                None => None
            }),
            added_on: network.added_on.into(),
            .. Self::default()
        }
    }
}
