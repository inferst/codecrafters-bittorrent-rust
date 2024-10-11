use core::{panic, str};
use std::collections::BTreeMap;

use crate::data::DataValue;

pub fn decode_string(bytes: Vec<u8>) -> (DataValue, Vec<u8>) {
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

    (DataValue::String(value.to_vec()), rest.to_vec())
}

pub fn decode_dictionary(bytes: Vec<u8>) -> (DataValue, Vec<u8>) {
    let mut list = vec![];
    let mut map = BTreeMap::new();

    let mut rest = bytes;

    while !rest.starts_with(&[b'e']) {
        let result = decode_value(rest);
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
            DataValue::String(str) => str::from_utf8(&str).unwrap().to_string(),
            _ => string,
        };

        map.insert(key.to_string(), value);
    }

    (DataValue::Dictionary(map), rest)
}

pub fn decode_value(bytes: Vec<u8>) -> (DataValue, Vec<u8>) {
    let (first, rest) = bytes.split_first().unwrap();

    match first {
        b'd' => decode_dictionary(rest.to_vec()),
        b'l' => decode_list(rest.to_vec()),
        b'i' => decode_integer(rest.to_vec()),
        b'0'..=b'9' => decode_string(bytes),
        _ => {
            panic!("Error");
        }
    }
}

fn decode_integer(bytes: Vec<u8>) -> (DataValue, Vec<u8>) {
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

    (DataValue::Integer(value.parse().unwrap()), rest)
}

pub fn decode_list(bytes: Vec<u8>) -> (DataValue, Vec<u8>) {
    let mut list = vec![];

    let mut rest = bytes;

    while !rest.starts_with(&[b'e']) {
        let result = decode_value(rest);
        list.push(result.0);
        rest = result.1;
    }

    rest.remove(0);

    (DataValue::List(list), rest)
}

pub fn decode_bencoded_value(bencoded_value: Vec<u8>) -> DataValue {
    decode_value(bencoded_value).0
}
