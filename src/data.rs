use core::{fmt, str};
use std::collections::BTreeMap;

use crate::decoder::decode_bencoded_value;

#[derive(Clone, Debug)]
pub enum DataValue {
    Dictionary(BTreeMap<String, DataValue>),
    List(Vec<DataValue>),
    String(Vec<u8>),
    Integer(usize),
}

impl DataValue {
    pub fn decode(bencoded_value: Vec<u8>) -> DataValue {
        decode_bencoded_value(bencoded_value)
    }

    pub fn get(&self, key: &str) -> &DataValue {
        match self {
            DataValue::Dictionary(entries) => entries.get(key).unwrap(),
            _ => self,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        match self {
            DataValue::Dictionary(entries) => {
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
            DataValue::String(string) => {
                let mut bytes = string.clone();
                let mut result = format!("{}:", bytes.len()).as_bytes().to_vec();
                result.append(&mut bytes);
                result
            }
            DataValue::Integer(number) => {
                return format!("i{number}e").as_bytes().to_vec();
            }
            DataValue::List(values) => {
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

impl fmt::Display for DataValue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataValue::String(string) => write!(fmt, "{}", str::from_utf8(string).unwrap()),
            DataValue::Integer(number) => write!(fmt, "{number}"),
            DataValue::List(_) => write!(fmt, "List"),
            DataValue::Dictionary(_) => write!(fmt, "{{}}"),
        }
    }
}
