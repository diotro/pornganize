use serde::{Deserialize, Serialize};
mod tables {
    use sqlx::FromRow;

    #[derive(Debug, FromRow)]
    pub struct GameDeveloper {
        id: i64,
        name: String,
        established: String,
    }

    #[derive(Debug, FromRow)]
    pub struct GameStudio {
        id: i64,
        name: String,
        established: String,
    }

    #[derive(Debug, FromRow)]
    pub struct Game {
        id: i64,
        name: String,
        released: String,
        language: String,
        path: String,
    }

    #[derive(Debug, FromRow)]
    pub struct GameTag {
        tag_id: i64,
        game_id: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    id: u64,
    name: String,
}
