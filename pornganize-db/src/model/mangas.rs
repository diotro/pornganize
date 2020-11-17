use serde::{Deserialize, Serialize};

mod tables {
    use sqlx::FromRow;
    #[derive(Debug, FromRow)]
    pub struct Manga {
        id: i64,
    }

    #[derive(Debug, FromRow)]
    pub struct MangaTag {
        tag_id: i64,
        manga_id: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manga {
    id: u64,
}
