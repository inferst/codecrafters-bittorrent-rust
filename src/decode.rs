pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    if encoded_value.chars().next().unwrap().is_ascii_digit() {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let number = usize::try_from(number).unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number];
        return serde_json::Value::String(string.to_string());
    }

    panic!("Unhandled encoded value: {encoded_value}");
}
