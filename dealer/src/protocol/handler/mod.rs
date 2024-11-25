use serde::{Deserialize, Serialize};



// 定义 PacketType 枚举
#[derive(Serialize, Deserialize)]
pub enum PacketType {
    Login,
}

pub mod login_handler;