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

impl From<&SiteMessage> for Site {
    fn from(msg: &SiteMessage) -> Self {
        Self {
            id: msg.id.clone(),
            name: msg.name.clone(),
            banner: msg.banner.clone(),
            logo: msg.logo.clone(),
            url: msg.url.clone(),
            description: msg.description.clone(),
            studio_id:
                if msg.studio_id.is_empty() { None }
                else { Some(msg.studio_id.clone()) },
            network_id:
                if msg.network_id.is_empty() { None }
                else { Some(msg.network_id.clone()) },
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

impl From<&Site> for SiteMessage {
    fn from(site: &Site) -> Self {
        Self {
            id: site.id.clone(),
            name: site.name.clone(),
            banner: site.banner.clone(),
            logo: site.logo.clone(),
            url: site.url.clone(),
            description: site.description.clone(),
            studio_id: String::from(site.studio_id.as_deref().unwrap_or("")),
            network_id: String::from(site.network_id.as_deref().unwrap_or("")),
            established: SingularPtrField::from_option(match site.established.as_ref() {
                Some(dt) => Some(dt.into()),
                None => None
            }),
            added_on: (&site.added_on).into(),
            .. Self::default()
        }
    }
}

impl Model<'_, SiteMessage> for Site {
    const TREE_NAME: &'static str = "sites";
    fn get_key(&self) -> &str { &self.id }
}
