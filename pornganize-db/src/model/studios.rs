use serde::{Deserialize, Serialize};

mod tables {
    use sqlx::FromRow;
    #[derive(Debug, FromRow)]
    pub struct Studio {
        id: i64,
        name: String,
        established: String,
        banner: String,
        logo: String,
    }

    #[derive(Debug, FromRow)]
    pub struct StudioTag {
        tag_id: i64,
        studio_id: String,
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Studio {
    id: u64,
}
