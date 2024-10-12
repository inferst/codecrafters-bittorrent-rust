use sha1::{Digest, Sha1};

use crate::data::DataValue;

pub struct Info {
    data: DataValue,
}

impl Info {
    pub fn from(data: &DataValue) -> Self {
        Info { data: data.clone() }
    }

    pub fn info_hash(&self) -> Vec<u8> {
        let info = self.data.get("info");
        let mut hasher = Sha1::new();
        hasher.update(info.encode());

        hasher.finalize().to_vec()
    }

    pub fn length(&self) -> String {
        let info = self.data.get("info");
        info.get("length").to_string()
    }

    pub fn print(&self) {
        let info = self.data.get("info");

        let info_hash = self.info_hash();
        let code = hex::encode(info_hash);

        println!("Tracker URL: {}", self.data.get("announce").value());
        println!("Length: {}", info.get("length"));
        println!("Info Hash: {code}");
        println!("Piece Length: {}", info.get("piece length"));

        let pieces = info.get("pieces");
        let mut piece = vec![];

        println!("Piece Hashes:");

        for byte in pieces.bytes() {
            if piece.len() == 20 {
                println!("{}", hex::encode(piece));
                piece = vec![];
            }

            piece.push(*byte);
        }

        println!("{}", hex::encode(piece));
    }
}
