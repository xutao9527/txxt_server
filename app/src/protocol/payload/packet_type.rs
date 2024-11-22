use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
pub enum PacketType {
    TestOne,
    TestTwo,
}


