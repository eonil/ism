// Note: Generated by AI assistant
use ism::text::Message;
use ism_derive::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[derive(Serialize, Deserialize)]
struct BasicMessage {
    msg: String,
    list: Vec<String>,
    data: HashMap<String, String>,
}

#[test]
fn test_derive_basic() {
    let msg = BasicMessage {
        msg: "hello".to_string(),
        list: vec!["a".to_string(), "b".to_string()],
        data: {
            let mut map = HashMap::new();
            map.insert("x".to_string(), "y".to_string());
            map
        },
    };

    let ism_msg = msg.serialize();
    let decoded = BasicMessage::deserialize(ism_msg).unwrap();
    assert_eq!(msg, decoded);
} 