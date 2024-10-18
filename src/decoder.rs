use core::{panic, str};
use std::collections::BTreeMap;

use crate::bencode::Bencode;

fn dictionary(bytes: Vec<u8>) -> (Bencode, Vec<u8>) {
    let mut list = vec![];
    let mut map = BTreeMap::new();

    let mut rest = bytes;

    while !rest.starts_with(&[b'e']) {
        let result = decode(rest);
        list.push(result.0);
        rest = result.1;
    }

    rest.remove(0);

    let len = list.len() / 2;

    for _ in 0..len {
        let value = list.pop().unwrap();
        let key = list.pop().unwrap();
        let string = key.to_string();

        let key = match key {
            Bencode::String(str) => str::from_utf8(&str).unwrap().to_string(),
            _ => string,
        };

        map.insert(key.to_string(), value);
    }

    (Bencode::Dictionary(map), rest)
}

fn list(bytes: Vec<u8>) -> (Bencode, Vec<u8>) {
    let mut list = vec![];

    let mut rest = bytes;

    while !rest.starts_with(&[b'e']) {
        let result = decode(rest);
        list.push(result.0);
        rest = result.1;
    }

    rest.remove(0);

    (Bencode::List(list), rest)
}

fn string(bytes: Vec<u8>) -> (Bencode, Vec<u8>) {
    let mut length = String::new();
    let mut rest = bytes.clone();

    for byte in bytes {
        rest.remove(0);

        if byte == b':' {
            break;
        }

        length.push(char::from(byte));
    }

    let number = length.parse::<usize>().unwrap();

    let (value, rest) = rest.split_at(number);

    (Bencode::String(value.to_vec()), rest.to_vec())
}

fn integer(bytes: Vec<u8>) -> (Bencode, Vec<u8>) {
    let mut value = vec![];
    let mut rest = bytes.clone();

    for byte in bytes {
        rest.remove(0);

        if byte == b'e' {
            break;
        }

        value.push(byte);
    }

    let value: String = value.iter().map(|x| char::from(*x)).collect();

    (Bencode::Integer(value.parse().unwrap()), rest)
}

pub fn decode(bytes: Vec<u8>) -> (Bencode, Vec<u8>) {
    let (first, rest) = bytes.split_first().unwrap();

    let value = String::from_utf8_lossy(&bytes);

    match first {
        b'd' => dictionary(rest.to_vec()),
        b'l' => list(rest.to_vec()),
        b'i' => integer(rest.to_vec()),
        b'0'..=b'9' => string(bytes),
        _ => {
            panic!("Decode error: {value}");
        }
    }
}
