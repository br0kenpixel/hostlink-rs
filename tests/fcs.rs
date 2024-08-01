use hostlink::protocol::{Message, MessageKind, MessageParams, NodeId};

#[test]
fn fcs_1() {
    let node = NodeId::new(0).unwrap();
    let kind = MessageKind::Test;
    let params = MessageParams::new();
    let command = Message::new(node, kind, params);

    let serialized = command.serialize().unwrap();
    let fcs = hostlink::protocol::fcs::fcs(&serialized[..serialized.len() - 4]).unwrap();

    assert_eq!(fcs.value(), 47);
}

#[test]
fn fcs_2() {
    let serialized = "@10RH0031000158*\r";
    let fcs = hostlink::protocol::fcs::fcs(&serialized[..serialized.len() - 4]).unwrap();

    assert_eq!(fcs.value(), 58);
}
