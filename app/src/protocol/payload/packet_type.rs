use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketType {
    TestOne,
    TestTwo,
}


