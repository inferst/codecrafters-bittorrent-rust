use std::{env, fs};

use data::DataValue;
use decoder::decode_bencoded_value;
use sha1::{Digest, Sha1};

mod data;
mod decoder;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let vector = encoded_value.as_bytes().to_vec();
        let decoded_value = decode_bencoded_value(vector);
        println!("{decoded_value}");
    } else if command == "info" {
        let filename = &args[2];
        let file_contents = fs::read(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {filename}");
            Vec::new()
        });
        let decoded_value = DataValue::decode(file_contents);
        let mut hasher = Sha1::new();
        let info = decoded_value.get("info");
        hasher.update(info.encode());
        let result = hasher.finalize();
        let code = hex::encode(result);
        println!("Tracker URL: {}", decoded_value.get("announce"));
        println!("Length: {}", info.get("length"));
        println!("Info Hash: {code}");
    } else {
        println!("unknown command: {}", args[1]);
    }
}
