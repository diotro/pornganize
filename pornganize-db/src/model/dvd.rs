use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dvd {
    id: u64,
    title: String,
    cover_img: String,
    back_img: String,
}
