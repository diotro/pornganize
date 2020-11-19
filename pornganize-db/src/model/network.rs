use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::{
    Model,
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
    studio_id: String,
    established: DateTime<Utc>,
    added_on: DateTime<Utc>,
}

impl From<NetworkMessage> for Network {
    fn from(msg: NetworkMessage) -> Self {
        let established: DateTime<Utc> = match msg.established.into_option() {
            Some(dt) => DateTime::from(&dt),
            None => Utc::now(),
        };
        let added_on: DateTime<Utc> = match msg.added_on.into_option() {
            Some(dt) => DateTime::from(&dt),
            None => Utc::now(),
        };
        Self {
            id: msg.id,
            name: msg.name,
            banner: msg.banner,
            logo: msg.logo,
            website: msg.website,
            description: msg.description,
            studio_id: msg.studio_id,
            established,
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
