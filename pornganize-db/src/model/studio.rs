use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::{
    Model,
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
    established: DateTime<Utc>,
    added_on: DateTime<Utc>,
}

//impl From<StudioMessage> for Studio {
    //fn from(msg: StudioMessage) -> Self {
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
            //website: msg.website,
            //description: msg.description,
            //established,
            //added_on,
        //}
    //}
//}

//impl Model<StudioMessage> for Studio {
    //fn tree_name() -> &'static str { "studios" }

    //fn get_key(&self) -> String {
        //self.id.clone()
    //}
//}
