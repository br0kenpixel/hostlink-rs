use hostlink::protocol::{fcs::fcs, Command, CommandKind, CommandParams, NodeId};

#[test]
fn serialize_1() {
    let node = NodeId::new(0).unwrap();
    let kind = CommandKind::Test;
    let params = CommandParams::new();
    let command = Command::new(node, kind, params);

    let serialized = command.serialize().unwrap();
    assert_eq!(
        serialized.as_ref(),
        format!("@00TS{}*\r", fcs("@00TS").unwrap()).as_str()
    );
}
