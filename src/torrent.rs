use std::{error::Error, net::Ipv4Addr};

use sha1::{Digest, Sha1};
use url::{form_urlencoded, Url};

use crate::{bencode::Bencode, peer::Peer};

pub struct Torrent {
    announce: Bencode,
    info: Bencode,
}

impl Torrent {
    pub fn from(data: &Bencode) -> Self {
        Torrent {
            announce: data.get("announce").clone(),
            info: data.get("info").clone(),
        }
    }

    pub fn info_hash(&self) -> Vec<u8> {
        let mut hasher = Sha1::new();
        hasher.update(self.info.encode());
        hasher.finalize().to_vec()
    }

    pub fn pieces(&self) -> Vec<u8> {
        self.info.get("pieces").bytes().to_vec()
    }

    pub fn length(&self) -> u32 {
        self.info.get("length").value().parse().unwrap()
    }

    pub fn piece_length(&self) -> u32 {
        self.info.get("piece length").value().parse().unwrap()
    }

    pub async fn get_peers(&self) -> Result<Vec<Peer>, Box<dyn Error>> {
        let info_hash: String = form_urlencoded::byte_serialize(&self.info_hash()).collect();

        let params = [
            ("peer_id", "12345678901234567890".to_string()),
            ("port", "6881".to_string()),
            ("uploaded", "0".to_string()),
            ("downloaded", "0".to_string()),
            ("left", self.length().to_string()),
            ("compact", "1".to_string()),
        ];

        let url = Url::parse_with_params(&self.announce.value(), &params).unwrap();

        let response = reqwest::get(format!("{url}&info_hash={info_hash}"))
            .await?
            .bytes()
            .await?;

        let data = Bencode::decode(response.to_vec());
        let peers = data.get("peers").bytes();
        let chunks = peers.chunks(6);

        let mut result = vec![];

        for chunk in chunks {
            let ip = Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]);
            let port = u16::from_be_bytes([chunk[4], chunk[5]]);
            result.push(Peer { ip, port });
        }

        Ok(result)
    }

    pub fn print(&self) {
        let info_hash = self.info_hash();

        println!("Tracker URL: {}", self.announce.value());
        println!("Length: {}", self.length());
        println!("Info Hash: {}", hex::encode(info_hash));
        println!("Piece Length: {}", self.piece_length());

        let mut piece = vec![];

        println!("Piece Hashes:");

        for byte in self.pieces() {
            if piece.len() == 20 {
                println!("{}", hex::encode(piece));
                piece = vec![];
            }

            piece.push(byte);
        }

        println!("{}", hex::encode(piece));
    }
}
