use hostlink::protocol::{Command, CommandKind, CommandParams, NodeId};

#[test]
fn deserialize_1() {
    let original = Command::new(
        NodeId::new(0).unwrap(),
        CommandKind::Test,
        CommandParams::new(),
    );

    let serialized = original.clone().serialize().unwrap();
    let deserialized = Command::parse(&serialized).unwrap();

    assert_eq!(deserialized, original);
}

#[test]
fn deserialize_2() {
    let original = Command::new(
        NodeId::new(65).unwrap(),
        CommandKind::DmAreaRead,
        vec!['0', 'a'].into(),
    );

    let serialized = original.clone().serialize().unwrap();
    let deserialized = Command::parse(&serialized).unwrap();

    assert_eq!(deserialized, original);
}
