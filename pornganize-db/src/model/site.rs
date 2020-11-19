use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::{
    Model,
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
    studio_id: String,
    network_id: String,
    established: DateTime<Utc>,
    added_on: DateTime<Utc>,
}

//impl From<SiteMessage> for Site {
    //fn from(msg: SiteMessage) -> Self {
        //let established: DateTime<Utc> = match msg.established.into_option() {
            //Some(dt) => DateTime::from(&dt),
            //None => Utc::now(),
        //};
        //let added_on: DateTime<Utc> = match msg.added_on.into_option() {
            //Some(dt) => DateTime::from(&dt),
            //None => Utc::now(),
        //};
        //Self {
            //id: msg.id,
            //name: msg.name,
            //banner: msg.banner,
            //logo: msg.logo,
            //url: msg.url,
            //description: msg.description,
            //studio_id: msg.studio_id,
            //network_id: msg.network_id,
            //established,
            //added_on,
        //}
    //}
//}

//impl Model<SiteMessage> for Site {
    //fn tree_name() -> &'static str { "sites" }

    //fn get_key(&self) -> String {
        //self.id.clone()
    //}
//}
