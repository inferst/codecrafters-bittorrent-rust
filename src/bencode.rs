use core::{fmt, str};
use std::collections::BTreeMap;

use crate::decoder::decode;

#[derive(Clone, Debug)]
pub enum Bencode {
    Dictionary(BTreeMap<String, Bencode>),
    List(Vec<Bencode>),
    String(Vec<u8>),
    Integer(isize),
}

impl Bencode {
    pub fn decode(bencoded_value: Vec<u8>) -> Bencode {
        decode(bencoded_value).0
    }

    pub fn get(&self, key: &str) -> &Bencode {
        match self {
            Bencode::Dictionary(entries) => entries.get(key).unwrap(),
            _ => self,
        }
    }

    pub fn value(&self) -> String {
        match self {
            Bencode::String(string) => str::from_utf8(string).unwrap().to_string(),
            _ => self.to_string(),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            Bencode::String(bytes) => bytes,
            _ => &[],
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        match self {
            Bencode::Dictionary(entries) => {
                let start = b'd';
                let end = b'e';
                let mut bytes = entries.iter().fold(vec![start], |acc, x| {
                    let mut result = vec![];
                    let bytes = x.0.as_bytes();
                    let key = format!("{}:{}", bytes.len(), x.0).as_bytes().to_vec();
                    result.extend(acc);
                    result.extend(key);
                    result.extend(x.1.encode());
                    result
                });
                bytes.push(end);
                bytes
            }
            Bencode::String(string) => {
                let mut bytes = string.clone();
                let mut result = format!("{}:", bytes.len()).as_bytes().to_vec();
                result.append(&mut bytes);
                result
            }
            Bencode::Integer(number) => {
                return format!("i{number}e").as_bytes().to_vec();
            }
            Bencode::List(values) => {
                let start = b'l';
                let end = b'e';
                let mut bytes = values.iter().fold(vec![start], |acc, x| {
                    let mut result = vec![];
                    result.extend(acc);
                    result.extend(x.encode());
                    result
                });
                bytes.push(end);
                bytes
            }
        }
    }
}

impl fmt::Display for Bencode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bencode::String(string) => write!(fmt, "\"{}\"", str::from_utf8(string).unwrap()),
            Bencode::Integer(number) => write!(fmt, "{number}"),
            Bencode::List(values) => {
                let strings: Vec<String> = values
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect();
                let string = strings.join(",");
                write!(fmt, "[{string}]")
            }
            Bencode::Dictionary(entries) => {
                let strings: Vec<String> = entries
                    .iter()
                    .map(|(key, value)| format!("\"{key}\":{value}"))
                    .collect();
                let string = strings.join(",");
                write!(fmt, "{{{string}}}")
            }
        }
    }
}
