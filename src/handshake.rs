use std::error::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{data::DataValue, info};

pub async fn connect(data: DataValue, peer: String) -> Result<(), Box<dyn Error>> {
    let info = info::Info::from(&data);
    let mut stream = TcpStream::connect(peer).await?;

    let mut buffer: Vec<u8> = vec![];

    buffer.push(19);
    buffer.extend("BitTorrent protocol".as_bytes());
    buffer.extend([0; 8]);
    buffer.extend(info.info_hash());
    buffer.extend([0; 20]);

    stream.write_all(&buffer).await?;

    stream.read_exact(&mut buffer).await?;

    let peer_id = &buffer.last_chunk::<20>().unwrap();

    println!("Peer ID: {}", hex::encode(peer_id));

    Ok(())
}
