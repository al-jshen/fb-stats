use crate::{Message, Participant};
use rayon::prelude::*;
use regex::Regex;
use serde_json::Value;
use std::fs::{read_dir, File, OpenOptions};
use std::io::BufReader;

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

pub fn get_message_file(fname: &str) -> Value {
    let f = File::open(fname).unwrap();
    let r = BufReader::new(f);
    serde_json::from_reader(r).unwrap()
}

pub fn get_messages(fname: &str) -> Vec<Message> {
    let mf = get_message_file(fname);
    serde_json::from_str(&mf["messages"].to_string()).unwrap()
}

pub fn get_participants(fname: &str) -> Vec<Participant> {
    let f = get_message_file(fname);
    serde_json::from_value(
        f.as_object()
            .unwrap()
            .get("participants")
            .unwrap()
            .to_owned(),
    )
    .unwrap()
}

pub fn merge_files(dir: &str, outfile: &str) {
    let rg = Regex::new(r"message_\d+\.json$").unwrap();
    let of = OpenOptions::new()
        .create(true)
        .append(true)
        .open(outfile)
        .unwrap();

    let mut all_messages: Vec<Message> = Vec::new();

    for f in read_dir(dir).unwrap() {
        let fp = f.unwrap().path();
        if rg.is_match(&fp.file_name().unwrap().to_str().unwrap()) {
            all_messages.extend(get_messages(fp.as_os_str().to_str().unwrap()));
        }
    }

    serde_json::to_writer(of, &all_messages).unwrap();
}

pub fn get_cleaned_messages(fname: &str) -> Vec<Message> {
    let f = File::open(fname).unwrap();
    let r = BufReader::new(f);
    serde_json::from_reader(r).unwrap()
}

pub fn get_all_participants(fname: &str) -> Vec<String> {
    let messages = get_cleaned_messages(fname);
    let mut parts = messages
        .into_par_iter()
        .map(|m| m.sender_name)
        .collect::<Vec<_>>();
    parts.sort_unstable();
    parts.dedup();
    parts
}
