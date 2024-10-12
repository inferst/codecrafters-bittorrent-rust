use std::{error::Error, net::Ipv4Addr};
use url::{form_urlencoded, Url};

use crate::{
    data::DataValue,
    info::{self},
};

pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16,
}

pub async fn get_peers(data: &DataValue) -> Result<Vec<Peer>, Box<dyn Error>> {
    let info = info::Info::from(&data);
    let url = data.get("announce").value();
    let length = info.length();
    let info_hash: String = form_urlencoded::byte_serialize(&info.info_hash()).collect();

    let params = [
        ("peer_id", "12345678901234567890".to_string()),
        ("port", "6881".to_string()),
        ("uploaded", "0".to_string()),
        ("downloaded", "0".to_string()),
        ("left", length),
        ("compact", "1".to_string()),
    ];

    let url = Url::parse_with_params(&url, &params).unwrap();

    let response = reqwest::get(format!("{url}&info_hash={info_hash}"))
        .await?
        .bytes()
        .await?;

    let data = DataValue::decode(response.to_vec());
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
