use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketPayload {
    TestOne,
    TestTwo,
}