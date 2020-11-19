use serde::{Deserialize, Serialize};
use super::{
    Model,
    DateTime,
    messages::studio::Studio as StudioMessage,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    avatar: Option<String>,
    banner: Option<String>,
    profile1: Option<String>,
    profile2: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareerStatus {
    started: Option<DateTime>,
    is_active: bool,
    retired: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    id: String,
    name: String,
    aliases: Vec<String>,
    name_patterns: Vec<String>,
    //gender: 'static &Gender,
    nationality: String,
    dob: String,
}
