//use serde_json;
use std::{env, fs};

use decoder::decode_bencoded_value;
use parser::parse;

mod decoder;
mod metainfo;
mod parser;

// Available if you need it!
// use serde_bencode

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let mut vector = encoded_value.as_bytes().to_vec();
        let decoded_value = decode_bencoded_value(&mut vector);
        println!("{decoded_value}");
    } else if command == "info" {
        let filename = &args[2];
        let mut file_contents = fs::read(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {filename}");
            Vec::new()
        });
        let decoded_value = decode_bencoded_value(&mut file_contents);
        parse(decoded_value);
    } else {
        println!("unknown command: {}", args[1]);
    }
}
