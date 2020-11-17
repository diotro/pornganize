use serde::{Deserialize, Serialize};

mod tables {
    use sqlx::FromRow;

    #[derive(Debug, FromRow)]
    pub struct Image {
        id: i64,
        title: String,
        path: String,
    }

    #[derive(Debug, FromRow)]
    pub struct ImageTag {
        tag_id: i64,
        image_id: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    id: u64,
    path: String,
}
