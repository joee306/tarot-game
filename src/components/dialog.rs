use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    fs,
    io::{prelude::*, BufReader},
};

#[derive(Component, Deserialize, Serialize, Debug)]
pub struct Dialog(Vec<Sentance>);

#[derive(Deserialize, Serialize, Debug)]
struct Sentance {
    owner: String,
    text: String,
}

impl Dialog {
    pub fn new(path: String) -> Dialog {
        let file = fs::File::open(path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();
        from_str(&content).unwrap()
    }
}
