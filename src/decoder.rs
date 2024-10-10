use std::{iter::Peekable, slice::Iter};

use serde_json::Map;

pub fn decode_string(chars: &mut Peekable<Iter<u8>>) -> Option<serde_json::Value> {
    if let Some(&char) = chars.peek() {
        if char.is_ascii_digit() {
            let mut length = String::new();

            loop {
                let char = chars.next()?;

                if *char == b':' {
                    break;
                }

                length.push(char::from(*char));
            }

            let number = length.parse::<usize>().unwrap();
            let list: Vec<usize> = (0..number).collect();
            let string = list
                .iter()
                .map(|_| char::from(*chars.next().unwrap()))
                .collect();

            return Some(serde_json::Value::String(string));
        }
    }

    None
}

pub fn decode_dictionary(chars: &mut Peekable<Iter<u8>>) -> Option<serde_json::Value> {
    if let Some(&char) = chars.peek() {
        if char::from(*char) == 'd' {
            let mut list = vec![];
            let mut map = Map::new();

            chars.next()?;

            while let Some(value) = decode_value(chars) {
                list.push(value);
            }

            if let Some(char) = chars.next() {
                assert!(*char == b'e', "Unhandled encoded value");
            }

            let len = list.len() / 2;

            for _ in 0..len {
                let value = list.pop().unwrap();
                let key = list.pop().unwrap().as_str().unwrap().to_string();

                map.insert(key, value);
            }

            return Some(serde_json::Value::Object(map));
        }
    }

    None
}

pub fn decode_value(chars: &mut Peekable<Iter<u8>>) -> Option<serde_json::Value> {
    let first = chars.peek().unwrap();

    match first {
        b'd' => {
            return decode_dictionary(chars);
        }
        b'l' => {
            return decode_list(chars);
        }
        b'i' => {
            return decode_integer(chars);
        }
        b'0'..=b'9' => {
            return decode_string(chars);
        }
        _ => {}
    }

    None
}

pub fn decode_integer(chars: &mut Peekable<Iter<u8>>) -> Option<serde_json::Value> {
    if let Some(&char) = chars.peek() {
        if *char == b'i' {
            let mut string = String::new();

            chars.next()?;

            loop {
                let char = chars.next()?;

                if *char == b'e' {
                    let number = string.parse().unwrap();
                    return Some(serde_json::Value::Number(number));
                }

                string.push(char::from(*char));
            }
        }
    }

    None
}

pub fn decode_list(chars: &mut Peekable<Iter<u8>>) -> Option<serde_json::Value> {
    if let Some(&char) = chars.peek() {
        if *char == b'l' {
            let mut list = vec![];

            chars.next()?;

            while let Some(value) = decode_value(chars) {
                list.push(value);
            }

            if let Some(char) = chars.next() {
                assert!(*char == b'e', "Unhandled encoded value");
            }

            return Some(serde_json::Value::Array(list));
        }
    }

    None
}

pub fn decode_bencoded_value(encoded_value: &mut Vec<u8>) -> serde_json::Value {
    let mut chars = encoded_value.iter().peekable();
    decode_value(&mut chars).unwrap()
}
