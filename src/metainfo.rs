pub struct MetaInfo {
    pub announce: String,
    pub length: String,
}

impl MetaInfo {
    pub fn from_json(json: serde_json::Value) -> Self {
        MetaInfo {
            announce: json.get("announce").unwrap().as_str().unwrap().to_string(),
            length: json.get("info").unwrap().get("length").unwrap().to_string(),
        }
    }
}
