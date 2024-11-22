use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Debug, Clone)]
pub enum MsgType {
    T1Req,
    T2Req,
}