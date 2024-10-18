use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

use tokio::task::JoinSet;

use crate::{
    bencode::Bencode,
    peer::{Peer, PieceStatus},
    torrent::{self, Torrent},
};

#[derive(Debug, Eq, PartialEq)]
pub struct Piece {
    pub hash: String,
    pub status: PieceStatus,
}

pub async fn load_piece(torrent: &Torrent, index: u32) -> Result<Vec<u8>, Box<dyn Error>> {
    let peers = torrent.get_peers().await?;

    let peer = peers.first().unwrap();

    let mut peer = Peer::create(peer.clone()).await;
    peer.init(torrent).await?;
    let piece = peer.load_piece(torrent, index).await?;

    Ok(piece)
}

pub async fn load_file(file: &mut File, data: Bencode) -> Result<(), Box<dyn Error>> {
    let torrent = torrent::Torrent::from(&data);
    let piece_hashes = torrent.piece_hashes();
    let peers = torrent.get_peers().await?;

    let mut set = JoinSet::new();

    let length = u32::try_from(piece_hashes.len()).unwrap();

    let mut pieces_map = HashMap::new();

    for (index, hash) in piece_hashes.iter().enumerate() {
        pieces_map.insert(
            u32::try_from(index).unwrap(),
            Piece {
                hash: hash.to_string(),
                status: PieceStatus::Waiting,
            },
        );
    }

    let pieces = Arc::new(Mutex::new(pieces_map));
    let result = Arc::new(Mutex::new(HashMap::new()));

    for peer_id in peers {
        let torrent = torrent.clone();
        let pieces = pieces.clone();
        let result = result.clone();

        set.spawn(async move {
            let mut peer = Peer::create(peer_id.clone()).await;
            peer.init(&torrent).await.unwrap();
            peer.load(pieces, result, &torrent).await.unwrap();
            peer_id
        });
    }

    while let Some(peer_id) = set.join_next().await {
        let peer_id = peer_id.unwrap();
        println!("Peer finished: {peer_id}");
    }

    let result = result.lock().unwrap();
    let pieces = pieces.lock().unwrap();

    dbg!(&result.keys());
    dbg!(&pieces);

    for index in 0..length {
        let piece = result.get(&index).unwrap();
        file.write_all(piece)?;
    }

    Ok(())
}
