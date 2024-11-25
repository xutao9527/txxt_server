use serde::{Deserialize, Serialize};
use crate::protocol::payload::packet_payload::PacketPayload;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestTwo {
    pub two: String,
}


impl From<TestTwo> for PacketPayload {
    fn from(t1: TestTwo) -> Self {
        PacketPayload::TestTwo(t1)
    }
}