use hostlink::protocol::{Message, MessageKind, MessageParams, NodeId};

#[test]
fn deserialize_1() {
    let original = Message::new(
        NodeId::new(0).unwrap(),
        MessageKind::Test,
        MessageParams::new(),
    );

    let serialized = original.clone().serialize().unwrap();
    let deserialized = Message::parse(&serialized).unwrap();

    assert_eq!(deserialized, original);
}

#[test]
fn deserialize_2() {
    let original = Message::new(
        NodeId::new(65).unwrap(),
        MessageKind::DmAreaRead,
        vec!['0', 'a'].into(),
    );

    let serialized = original.clone().serialize().unwrap();
    let deserialized = Message::parse(&serialized).unwrap();

    assert_eq!(deserialized, original);
}
