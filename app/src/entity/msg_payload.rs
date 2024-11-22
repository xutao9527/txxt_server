use serde::{Deserialize, Serialize};
use crate::entity::msg_req::{T1Req, T2Req};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum MsgPayload {
    T1Req(T1Req),
    T2Req(T2Req),
}
