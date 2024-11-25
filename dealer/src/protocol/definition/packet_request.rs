use serde::{Deserialize, Serialize};
use crate::protocol::handler::PacketType;
use crate::protocol::payload::PacketPayload;


#[derive(Serialize, Deserialize)]
pub struct PacketRequest {
    pub packet_type: PacketType,
    pub packet_payload: PacketPayload,
}







