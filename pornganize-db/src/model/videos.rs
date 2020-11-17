use serde::{Deserialize, Serialize};

mod tables {
    use sqlx::FromRow;

    #[derive(Debug, FromRow)]
    pub struct Video {
        id: i64,
        path: String,
        title: String,
        resolution_height: i32,
        resolution_width: i32,
        framerate: i32,
        acodec: String,
        vcodec: String,
        thumbnail: String,
        site_id: Option<i64>,
        studio_id: Option<i64>,
    }

    #[derive(Debug, FromRow)]
    pub struct VideoTag {
        tag_id: i64,
        video_id: String,
    }

    #[derive(Debug, FromRow)]
    pub struct ScreenCap {
        id: i64,
        video_id: i64,
        path: String,
        time: i64,
    }

    #[derive(Debug, FromRow)]
    pub struct VideoMarker {
        id: i64,
        video_id: i64,
        time: i64,
    }

    #[derive(Debug, FromRow)]
    pub struct VideoMarkerTag {
        tag_id: i64,
        name: String,
        video_marker_id: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Marker {
    pub id: u32,
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
}
