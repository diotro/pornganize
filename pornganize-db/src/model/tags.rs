use serde::{Deserialize, Serialize};

mod tables {
    use sqlx::FromRow;

    #[derive(Debug, FromRow)]
    pub struct Tag {
        id: i64,
        name: String,
        applicable_to: i32,
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    id: u64,
}
