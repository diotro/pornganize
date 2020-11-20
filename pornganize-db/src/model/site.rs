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

impl<'a> From<SiteMessage> for &'a Site {
    fn from(msg: SiteMessage) -> Self {
        &&Site {
            id: msg.id,
            name: msg.name,
            banner: msg.banner,
            logo: msg.logo,
            url: msg.url,
            description: msg.description,
            studio_id:
                if msg.studio_id.is_empty() { None }
                else { Some(msg.studio_id) },
            network_id:
                if msg.network_id.is_empty() { None }
                else { Some(msg.network_id) },
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
            id: site.id,
            name: site.name,
            banner: site.banner,
            logo: site.logo,
            url: site.url,
            description: site.description,
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

pub struct SiteModeler;

impl<'a> Modeler<'a> for SiteModeler {
    type Model = Site;
    type Message = SiteMessage;
    const TREE_NAME: &'static str = "custom-fields";
    fn get_key(model: &'a Self::Model) -> &'a str { &model.id }
}
