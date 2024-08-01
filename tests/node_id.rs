use hostlink::protocol::{NodeId, ProtocolError};
use std::ops::Deref;

#[test]
fn node_id_1() {
    assert_eq!(NodeId::new(100), Err(ProtocolError::IllegalNodeId(100)));
}

#[test]
fn node_id_2() {
    assert_eq!(NodeId::new(1).unwrap().deref(), &1);
}
