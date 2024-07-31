use hostlink::protocol::{Error, NodeId};
use std::ops::Deref;

#[test]
fn node_id_1() {
    assert_eq!(NodeId::new(100), Err(Error::IllegalNodeId(100)));
}

#[test]
fn node_id_2() {
    assert_eq!(NodeId::new(1).unwrap().deref(), &1);
}
