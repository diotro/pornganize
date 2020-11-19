use serde::{Deserialize, Serialize};
use protobuf::SingularPtrField;
use super::{
    Model,
    DateTime,
    messages::site::Site as SiteMessage,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    id: String,
    name: String,
    banner: String,
    logo: String,
    url: String,
    description: String,
    studio_id: Option<String>,
    network_id: Option<String>,
    established: Option<DateTime>,
    added_on: DateTime,
}

impl From<SiteMessage> for Site {
    fn from(msg: SiteMessage) -> Self {
        let added_on: DateTime = match msg.added_on.into_option() {
            Some(dt) => DateTime::from(dt),
            None => DateTime::now(),
        };
        Self {
            id: msg.id,
            name: msg.name,
            banner: msg.banner,
            logo: msg.logo,
            url: msg.url,
            description: msg.description,
            studio_id: if msg.studio_id.is_empty() { None } else { Some(msg.studio_id) },
            network_id: if msg.network_id.is_empty() { None } else { Some(msg.network_id) },
            established: match msg.established.into_option() {
                Some(dt) => Some(DateTime::from(dt)),
                None => None,
            },
            added_on,
        }
    }
}

impl Model<SiteMessage> for Site {
    fn tree_name() -> &'static str { "sites" }
    fn get_key(&self) -> String { self.id.clone() }
}

impl From<Site> for SiteMessage {
    fn from(site: Site) -> Self {
        Self {
            id: site.id,
            name: site.name,
            banner: site.banner,
            logo: site.logo,
            url: site.url,
            description: site.description,
            studio_id: site.studio_id.unwrap_or(String::from("")),
            network_id: site.network_id.unwrap_or(String::from("")),
            established: SingularPtrField::from_option(match site.established {
                Some(dt) => Some(dt.into()),
                None => None
            }),
            added_on: site.added_on.into(),
            .. Self::default()
        }
    }
}
