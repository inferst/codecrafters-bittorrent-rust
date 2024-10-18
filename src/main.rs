use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
};

use bencode::Bencode;
use handshake::handshake;
use tokio::net::TcpStream;
use torrent::Torrent;

mod bencode;
mod decoder;
mod download;
mod handshake;
mod peer;
mod torrent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let vector = encoded_value.as_bytes().to_vec();
        let decoded_value = Bencode::decode(vector);
        println!("{decoded_value}");
    } else if command == "info" {
        let filename = &args[2];
        let file_contents = fs::read(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {filename}");
            Vec::new()
        });
        let data = Bencode::decode(file_contents);
        let info = torrent::Torrent::from(&data);
        info.print();
    } else if command == "peers" {
        let filename = &args[2];
        let file_contents = fs::read(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {filename}");
            Vec::new()
        });
        let data = Bencode::decode(file_contents);
        let torrent = Torrent::from(&data);
        let peers = torrent.get_peers().await.unwrap();
        for peer in peers {
            println!("{peer}");
        }
    } else if command == "handshake" {
        let filename = &args[2];
        let file_contents = fs::read(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {filename}");
            Vec::new()
        });
        let peer = &args[3].to_string();
        let data = Bencode::decode(file_contents);
        let torrent = Torrent::from(&data);
        let mut stream = TcpStream::connect(peer).await?;
        let peer_id = handshake(&mut stream, &torrent).await?;
        println!("Peer ID: {peer_id}");
    } else if command == "download_piece" {
        let output = &args[3];

        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(output)
            .unwrap();

        file.write_all(&[]).unwrap();

        let filename = &args[4];
        let file_contents = fs::read(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {filename}");
            Vec::new()
        });

        let index = &args[5].parse().unwrap();
        let data = Bencode::decode(file_contents);
        let torrent = Torrent::from(&data);

        let piece = download::load_piece(&torrent, *index).await?;
        file.write_all(&piece)?;
    } else if command == "download" {
        let output = &args[3];

        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(output)
            .unwrap();

        file.write_all(&[]).unwrap();

        let filename = &args[4];
        let file_contents = fs::read(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {filename}");
            Vec::new()
        });

        let data = Bencode::decode(file_contents);

        download::load_file(&mut file, data).await?;
    } else {
        println!("unknown command: {}", args[1]);
    }

    Ok(())
}
