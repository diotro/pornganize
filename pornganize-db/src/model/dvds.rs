use serde::{Deserialize, Serialize};
mod tables {
    use sqlx::FromRow;
    #[derive(Debug, FromRow)]
    pub struct Dvd {
        id: i64,
        title: String,
        studio_id: i64,
        cover_img: String,
        back_img: String,
    }

    #[derive(Debug, FromRow)]
    pub struct DvdTag {
        tag_id: i64,
        dvd_id: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dvd {
    id: u64,
    title: String,
    cover_img: String,
    back_img: String,
}
