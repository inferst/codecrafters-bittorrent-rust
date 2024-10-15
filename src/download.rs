use std::{error::Error, fs::File, io::Write};

use bytes::BufMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{bencode::Bencode, handshake::handshake, torrent};

pub async fn read_message(stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut length = [0; 4];
    stream.read_exact(&mut length).await?;

    let mut message = vec![0; u32::from_be_bytes(length) as usize];
    stream.read_exact(&mut message).await?;

    Ok(message)
}

pub async fn send_message(
    stream: &mut TcpStream,
    message_id: u8,
    payload: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    let length = payload.len() + 1;
    let length = u32::try_from(length).unwrap();

    let mut message = vec![];
    message.put_u32(length);
    message.put_u8(message_id);

    message.extend(payload);

    stream.write_all(&message).await?;

    Ok(())
}

const BLOCK_SIZE: u32 = 16 * 1024;

pub async fn download_piece(
    file: &mut File,
    data: Bencode,
    index: u32,
) -> Result<(), Box<dyn Error>> {
    let torrent = torrent::Torrent::from(&data);
    let peers = torrent.get_peers().await?;
    let peer = peers.first().unwrap();

    let mut stream = TcpStream::connect(peer.to_string()).await?;

    handshake(&mut stream, data).await?;

    let message = read_message(&mut stream).await?;

    assert!(message[0] == 5);

    send_message(&mut stream, 2, vec![]).await?;

    let message = read_message(&mut stream).await?;

    assert!(message[0] == 1);

    let file_length = torrent.length();
    let piece_length = torrent.piece_length();
    let piece_size = piece_length.min(file_length - piece_length * index);

    println!("File length: {file_length}");
    println!("Piece length: {piece_length}");
    println!("Piece size: {piece_size}");

    let mut piece_rest = piece_size;
    let blocks = (piece_size + BLOCK_SIZE - 1) / BLOCK_SIZE;

    dbg!(blocks);

    for block in 0..blocks {
        let block_size = BLOCK_SIZE.min(piece_rest);
        let mut payload = vec![];
        payload.put_u32(index);
        payload.put_u32(block * BLOCK_SIZE);
        payload.put_u32(block_size);

        println!("Block size: {block_size}");

        send_message(&mut stream, 6, payload).await?;

        let message = read_message(&mut stream).await?;

        assert!(message[0] == 7);

        file.write_all(&message[9..])?;

        piece_rest -= block_size;
    }

    let length = file.metadata().unwrap().len();

    println!("File size: {length}");

    Ok(())
}
