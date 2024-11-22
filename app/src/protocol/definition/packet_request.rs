use serde::{Deserialize, Serialize};
use crate::protocol::payload::packet_payload::PacketPayload;
use crate::protocol::payload::packet_type::PacketType;


#[derive(Serialize, Deserialize, Debug)]
pub struct PacketRequest {
    pub packet_type: PacketType,
    pub packet_payload: PacketPayload,
}