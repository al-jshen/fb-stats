use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct Media {
    pub uri: String,
    pub creation_timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct Sticker {
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Share {
    pub link: String,
}

#[derive(Deserialize, Debug)]
pub struct Reaction {
    pub reaction: String,
    pub actor: String,
}

#[derive(Deserialize, Debug)]
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

pub fn load_message_file(fname: String) -> Value {
    let f = File::open(fname).unwrap();
    let r = BufReader::new(f);
    serde_json::from_reader(r).unwrap()
}

pub fn load_messages(fname: String) -> Vec<Message> {
    let mf = load_message_file(fname);
    serde_json::from_str(&mf["messages"].to_string()).unwrap()
}

pub fn get_data_keys() -> (Vec<String>, Vec<String>) {
    let f = File::open(
        "/home/js/programs/fb-stats/data/messages/inbox/finessedkeeners_dbgu9woqpg/message_1.json",
    )
    .unwrap();
    let r = BufReader::new(f);
    let v: Value = serde_json::from_reader(r).unwrap();
    let dfields: Vec<String> = v
        .as_object()
        .unwrap()
        .keys()
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    let mut mfields: Vec<String> = Vec::new();

    for m in v["messages"].as_array().unwrap() {
        for k in m.as_object().unwrap().keys().collect::<Vec<_>>() {
            if !mfields.contains(k) {
                mfields.push(k.into());
            }
        }
    }

    (dfields, mfields)
}
