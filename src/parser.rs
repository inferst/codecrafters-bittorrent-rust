use crate::metainfo::MetaInfo;

pub fn parse(value: serde_json::Value) {
    let info = MetaInfo::from_json(value);

    println!("Tracker URL: {}", info.announce);
    println!("Length: {}", info.length);
}
