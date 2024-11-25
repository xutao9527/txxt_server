use serde::{Deserialize, Serialize};
use crate::protocol::payload::test_one::TestOne;
use crate::protocol::payload::test_two::TestTwo;

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketPayload {
    TestOne(TestOne),
    TestTwo(TestTwo),
}