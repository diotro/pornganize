use serde::{Deserialize, Serialize};
use protobuf::{
    RepeatedField,
    SingularPtrField,
    Message,
    ProtobufEnum,
};
use super::messages::common::{
    ApplicableTo as ApplicableToMessage,
    Gender as GenderMessage,
};

mod datetime;
pub use datetime::*;
mod vecs;
pub use vecs::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Gender {
    Undefined = 0,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApplicableTo {
    Any = 0,
    Actor = 1,
    Network = 2,
    Site = 3,
    Studio = 4,
    Video = 5,
    Artist = 6,
    Character = 7,
    GameDeveloper = 8,
    Game = 9,
    GameStudio = 10,
}

impl From<&ApplicableToMessage> for ApplicableTo {
    fn from(applicable_to: &ApplicableToMessage) -> Self {
        match applicable_to {
            ApplicableToMessage::ANY => ApplicableTo::Any,
            ApplicableToMessage::ACTOR => ApplicableTo::Actor,
            ApplicableToMessage::NETWORK => ApplicableTo::Network,
            ApplicableToMessage::SITE => ApplicableTo::Site,
            ApplicableToMessage::STUDIO => ApplicableTo::Studio,
            ApplicableToMessage::VIDEO => ApplicableTo::Video,
            ApplicableToMessage::ARTIST => ApplicableTo::Artist,
            ApplicableToMessage::CHARACTER => ApplicableTo::Character,
            ApplicableToMessage::GAME_DEVELOPER => ApplicableTo::GameDeveloper,
            ApplicableToMessage::GAME => ApplicableTo::Game,
            ApplicableToMessage::GAME_STUDIO => ApplicableTo::GameStudio,
        }
    }
}

impl From<&ApplicableTo> for ApplicableToMessage {
    fn from(applicable_to: &ApplicableTo) -> Self {
        match applicable_to {
            ApplicableTo::Any => ApplicableToMessage::ANY,
            ApplicableTo::Actor => ApplicableToMessage::ACTOR,
            ApplicableTo::Network => ApplicableToMessage::NETWORK,
            ApplicableTo::Site => ApplicableToMessage::SITE,
            ApplicableTo::Studio => ApplicableToMessage::STUDIO,
            ApplicableTo::Video => ApplicableToMessage::VIDEO,
            ApplicableTo::Artist => ApplicableToMessage::ARTIST,
            ApplicableTo::Character => ApplicableToMessage::CHARACTER,
            ApplicableTo::GameDeveloper => ApplicableToMessage::GAME_DEVELOPER,
            ApplicableTo::Game => ApplicableToMessage::GAME,
            ApplicableTo::GameStudio => ApplicableToMessage::GAME_STUDIO,
        }
    }
}
