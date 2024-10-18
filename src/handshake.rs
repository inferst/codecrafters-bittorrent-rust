use std::error::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::torrent::Torrent;

pub async fn handshake(
    stream: &mut TcpStream,
    torrent: &Torrent,
) -> Result<String, Box<dyn Error>> {
    let mut buffer: Vec<u8> = vec![];

    buffer.push(19); // BitTorrent protocol
    buffer.extend("BitTorrent protocol".as_bytes());
    buffer.extend([0; 8]); // reserved
    buffer.extend(torrent.info_hash());
    buffer.extend([0; 20]); // peer id

    stream.write_all(&buffer).await?;
    stream.read_exact(&mut buffer).await?;

    let peer_id = buffer.last_chunk::<20>().unwrap();

    Ok(hex::encode(peer_id))
}
