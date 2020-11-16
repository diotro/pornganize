use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Marker {
    pub label: String,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resolution {
    pub height: u16,
    pub width: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub id: u64,
    pub path: String,
    pub title: String,
    pub resolution: Resolution,
    pub vcodec: String,
    pub acodec: String,
    pub duration: u64,
    pub framerate: i16,
    pub markers: Vec<Marker>,
    pub actors: Vec<u64>,
    pub thumbnails: Vec<u64>,
    pub tags: Vec<u64>,
    pub studio: u64,
    pub site: u64,
}

impl Video {
    pub fn from_path(path: &str) -> Self {
        unimplemented!();
    }
}