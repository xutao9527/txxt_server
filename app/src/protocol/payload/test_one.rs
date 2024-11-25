use serde::{Deserialize, Serialize};
use crate::protocol::payload::packet_payload::PacketPayload;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestOne {
    pub one: String,
}

impl From<TestOne> for PacketPayload {
    fn from(t1: TestOne) -> Self {
        PacketPayload::TestOne(t1)
    }
}