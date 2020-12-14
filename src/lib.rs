mod utils;

use serde::{Deserialize, Serialize};
pub use utils::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    pub uri: String,
    pub creation_timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Share {
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
    pub reaction: String,
    pub actor: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub sender_name: String,
    pub timestamp_ms: u64,
    pub r#type: String,
    pub content: Option<String>,
    pub photos: Option<Vec<Media>>,
    pub sticker: Option<Sticker>,
    pub reaction: Option<Vec<Reaction>>,
    pub audio_files: Option<Vec<Media>>,
    pub share: Option<Share>,
}
