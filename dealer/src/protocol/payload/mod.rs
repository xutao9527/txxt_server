use serde::{Deserialize, Serialize};
use crate::protocol::payload::login::LoginReq;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]  // 使用 untagged 来处理不同类型的 payload
pub enum PacketPayload {
    Login(LoginReq),
}

pub mod login;