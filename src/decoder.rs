pub fn decode_string(encoded_value: &str) -> serde_json::Value {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let number = usize::try_from(number).unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number];
    serde_json::Value::String(string.to_string())
}

pub fn decode_integer(encoded_value: &str) -> serde_json::Value {
    // Example: "i52e" -> "52"
    let len = encoded_value.len();
    let str = &encoded_value[1..len - 1];
    let number = str.parse().unwrap();
    serde_json::Value::Number(number)
}

pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    if encoded_value.starts_with('i') {
        return decode_integer(encoded_value);
    } else if encoded_value.chars().next().unwrap().is_ascii_digit() {
        return decode_string(encoded_value);
    }

    panic!("Unhandled encoded value: {encoded_value}");
}
