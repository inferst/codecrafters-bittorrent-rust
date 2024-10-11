use sha1::{Digest, Sha1};

use crate::data::DataValue;

pub fn print(data: &DataValue) {
    let info = data.get("info");

    let mut hasher = Sha1::new();
    hasher.update(info.encode());
    let result = hasher.finalize();
    let code = hex::encode(result);

    println!("Tracker URL: {}", data.get("announce").value());
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
