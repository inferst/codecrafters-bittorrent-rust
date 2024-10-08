use std::{iter::Peekable, str::Chars};

pub fn decode_string_from_chars(chars: &mut Peekable<Chars>) -> Option<serde_json::Value> {
    if let Some(&char) = chars.peek() {
        if char.is_ascii_digit() {
            let mut length = String::new();

            loop {
                let char = chars.next()?;

                if char == ':' {
                    break;
                }

                length.push(char);
            }

            let number = length.parse::<usize>().unwrap();
            let list: Vec<usize> = (0..number).collect();
            let string = list.iter().map(|_| chars.next().unwrap()).collect();

            return Some(serde_json::Value::String(string));
        }
    }

    None
}

pub fn decode_value_from_chars(chars: &mut Peekable<Chars>) -> Option<serde_json::Value> {
    if matches!(chars.peek(), Some(&char) if char == 'l') {
        return decode_list_from_chars(chars);
    } else if let Some(integer) = decode_integer_from_chars(chars) {
        return Some(integer);
    } else if let Some(string) = decode_string_from_chars(chars) {
        return Some(string);
    }

    None
}

pub fn decode_integer_from_chars(chars: &mut Peekable<Chars>) -> Option<serde_json::Value> {
    if let Some(&char) = chars.peek() {
        if char == 'i' {
            let mut string = String::new();

            chars.next()?;

            loop {
                let char = chars.next()?;

                if char == 'e' {
                    let number = string.parse().unwrap();
                    return Some(serde_json::Value::Number(number));
                }

                string.push(char);
            }
        }
    }

    None
}

pub fn decode_list_from_chars(chars: &mut Peekable<Chars>) -> Option<serde_json::Value> {
    if let Some(&char) = chars.peek() {
        if char == 'l' {
            let mut list = vec![];

            chars.next()?;

            while let Some(value) = decode_value_from_chars(chars) {
                list.push(value);
            }

            if let Some(char) = chars.next() {
                assert!(char == 'e', "Unhandled encoded value");
            }

            return Some(serde_json::Value::Array(list));
        }
    }

    None
}

pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let mut chars = encoded_value.chars().peekable();

    if encoded_value.starts_with('l') {
        decode_list_from_chars(&mut chars).unwrap()
    } else {
        decode_value_from_chars(&mut chars).unwrap()
    }
}
