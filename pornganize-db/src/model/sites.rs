use serde::{Deserialize, Serialize};

mod tables {
    use sqlx::FromRow;

    #[derive(Debug, FromRow)]
    pub struct Network {
        id: i64,
        name: String,
        established: String,
        url: String,
        studio_id: i64,
        description: String,
    }

    #[derive(Debug, FromRow)]
    pub struct NetworkTag {
        tag_id: i64,
        network_id: String,
    }

    #[derive(Debug, FromRow)]
    pub struct Site {
        id: i64,
        name: String,
        established: String,
        url: String,
        studio_id: i64,
        network_id: i64,
        description: String,
    }

    #[derive(Debug, FromRow)]
    pub struct SiteTag {
        tag_id: i64,
        site_id: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sites {
    id: u64,
}
