use serde::{Deserialize, Serialize};

mod tables {
    use sqlx::FromRow;
    #[derive(Debug, FromRow)]
    struct Actor {
        id: i64,
        str_id: String,
        name: String,
        gender: String,
        nationality: String,
        dob: i64,
        description: String;
    }
    #[derive(Debug, FromRow)]
    struct CustomActorData {
        id: i64,
        name: String,
    }
    #[derive(Debug, FromRow)]
    struct CustomActorDataValue {
        id: i64,
        actor_id: i64,
        custom_data_id: i64,
        facet: bool,
        value: String,
    }
    #[derive(Debug, FromRow)]
    struct ActorAlias {
        id: i64,
        actor_id: i64,
        alias: String,
    }
    #[derive(Debug, FromRow)]
    struct ActorNamePattern {
        id: i64,
        actor_id: i64,
        pattern: String,
    }

    #[derive(Debug, FromRow)]
    pub struct ActorTag {
        tag_id: i64,
        actor_id: String,
    }
}

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
