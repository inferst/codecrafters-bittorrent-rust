use std::{env, fs};

use data::DataValue;
use decoder::decode_bencoded_value;

mod data;
mod info;
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
        let data = DataValue::decode(file_contents);
        info::print(&data);
    } else {
        println!("unknown command: {}", args[1]);
    }
}
