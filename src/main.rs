use std::{env, error::Error, fs};

use data::DataValue;
use decoder::decode_bencoded_value;
use peers::get_peers;

mod data;
mod decoder;
mod info;
mod peers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
        let info = info::Info::from(&data);
        info.print();
    } else if command == "peers" {
        let filename = &args[2];
        let file_contents = fs::read(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {filename}");
            Vec::new()
        });
        let data = DataValue::decode(file_contents);
        let peers = get_peers(&data).await.unwrap();
        for peer in peers {
            println!("{}:{}", peer.ip, peer.port);
        }
    } else {
        println!("unknown command: {}", args[1]);
    }

    Ok(())
}
