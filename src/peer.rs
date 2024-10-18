use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};

use bytes::BufMut;
use sha1::{Digest, Sha1};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{download::Piece, torrent::Torrent};

const BLOCK_SIZE: u32 = 16 * 1024;

pub struct Peer {
    stream: TcpStream,
}

#[derive(PartialEq, Eq, Debug)]
pub enum PieceStatus {
    Waiting,
    Started,
    Loaded,
}

impl Peer {
    pub async fn create(peer: String) -> Self {
        let stream = TcpStream::connect(peer.clone()).await.unwrap();

        Peer { stream }
    }

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

    pub async fn read_message(stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut length = [0; 4];
        stream.read_exact(&mut length).await?;

        let mut message = vec![0; u32::from_be_bytes(length) as usize];
        stream.read_exact(&mut message).await?;

        Ok(message)
    }

    pub fn prepare_message(message_id: u8, payload: Vec<u8>) -> Vec<u8> {
        let length = payload.len() + 1;
        let length = u32::try_from(length).unwrap();

        let mut message = vec![];

        message.put_u32(length);
        message.put_u8(message_id);

        message.extend(payload);

        message
    }

    pub async fn send_message(
        stream: &mut TcpStream,
        message: Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        stream.write_all(&message).await?;

        Ok(())
    }

    pub async fn init(&mut self, torrent: &Torrent) -> Result<(), Box<dyn Error>> {
        println!("handshake");
        Peer::handshake(&mut self.stream, torrent).await?;

        println!("read message 1");
        let message = Peer::read_message(&mut self.stream).await?;

        assert!(message[0] == 5);

        println!("send message 1");
        let message = Peer::prepare_message(2, vec![]);
        Peer::send_message(&mut self.stream, message).await?;

        println!("read message 2");
        let message = Peer::read_message(&mut self.stream).await?;

        assert!(message[0] == 1);

        Ok(())
    }

    pub async fn load_piece(
        &mut self,
        torrent: &Torrent,
        index: u32,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let file_length = torrent.length();
        let piece_length = torrent.piece_length();
        let piece_size = piece_length.min(file_length - piece_length * index);

        println!("File length: {file_length}");
        println!("Piece length: {piece_length}");
        println!("Piece size: {piece_size}");

        let mut piece_rest = piece_size;
        let mut blocks = piece_size.div_ceil(BLOCK_SIZE);

        println!("Blocks: {blocks}");

        let mut piece = vec![];

        let block_count = 5;
        let mut block = 0;

        while blocks > 0 {
            let count = if blocks / block_count >= 1 {
                block_count
            } else {
                blocks
            };

            println!("Count: {count}");

            let mut messages = vec![];

            for _ in 0..count {
                let block_size = BLOCK_SIZE.min(piece_rest);
                let mut payload = vec![];
                payload.put_u32(index);
                payload.put_u32(block * BLOCK_SIZE);
                payload.put_u32(block_size);

                println!("Block size: {block_size}");

                messages.extend(Peer::prepare_message(6, payload));

                piece_rest -= block_size;

                block += 1;
            }

            Peer::send_message(&mut self.stream, messages).await?;

            for _ in 0..count {
                let message = Peer::read_message(&mut self.stream).await?;
                assert!(message[0] == 7);

                piece.extend(&message[9..]);
            }

            blocks -= count;
        }

        Ok(piece)
    }

    pub async fn load(
        &mut self,
        data: Arc<Mutex<HashMap<u32, Piece>>>,
        result: Arc<Mutex<HashMap<u32, Vec<u8>>>>,
        torrent: &Torrent,
    ) -> Result<(), Box<dyn Error>> {
        let index = {
            let mut index = 0;
            let mut found = false;

            let mut pieces = data.lock().unwrap();

            for (piece_index, piece) in pieces.iter_mut() {
                if piece.status == PieceStatus::Waiting {
                    piece.status = PieceStatus::Started;
                    index = *piece_index;
                    found = true;
                    break;
                }
            }

            if !found {
                return Ok(());
            }

            index
        };

        let piece = self.load_piece(torrent, index).await?;

        {
            let mut pieces = data.lock().unwrap();
            let data_piece = pieces.get_mut(&index).unwrap();

            let mut hasher = Sha1::new();
            hasher.update(&piece);
            let hash = hasher.finalize().to_vec();

            if data_piece.hash == hex::encode(hash) {
                let mut result = result.lock().unwrap();
                result.insert(index, piece);

                data_piece.status = PieceStatus::Loaded;
            }
        };

        Box::pin(self.load(data.clone(), result.clone(), torrent)).await
    }
}
