use std::{fmt, net::Ipv4Addr};

pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl fmt::Display for Peer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}:{}", self.ip, self.port)
    }
}
