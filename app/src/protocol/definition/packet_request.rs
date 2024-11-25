use serde::{Deserialize, Serialize};

use crate::protocol::definition::packet_payload::PacketPayload;
use crate::protocol::definition::packet_type::PacketType;


// 定义通用的 PacketRequest，`packet_payload` 使用 `serde_json::Value` 处理
#[derive(Serialize, Deserialize)]
pub struct PacketRequest {
    pub packet_type: PacketType,
    pub packet_payload: PacketPayload,  // 使用 Value 类型来接收任何类型的 payload
}







