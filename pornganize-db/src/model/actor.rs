use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    id: u64,
    name: String,
    aliases: Vec<String>,
    name_patterns: Vec<String>,
    //gender: 'static &Gender,
    nationality: String,
    dob: String,
}
