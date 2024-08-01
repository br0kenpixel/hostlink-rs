use hostlink::protocol::{fcs::fcs, Message, MessageKind, MessageParams, NodeId};

#[test]
fn serialize_1() {
    let node = NodeId::new(0).unwrap();
    let kind = MessageKind::Test;
    let params = MessageParams::new();
    let command = Message::new(node, kind, params);

    let serialized = command.serialize().unwrap();
    assert_eq!(
        serialized.as_ref(),
        format!("@00TS{}*\r", fcs("@00TS").unwrap()).as_str()
    );
}
