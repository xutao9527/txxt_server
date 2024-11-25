use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum PacketType {
    TestOne,
    TestTwo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PacketRequest<P> {
    pub packet_type: PacketType,
    pub packet_payload: P,
}

struct AsyncPacketProcessor {}
impl AsyncPacketProcessor {
    fn process<P>(packet: PacketRequest<P>)
        where
            P: Serialize + Deserialize<'static> + std::fmt::Debug,
    {
        match packet.packet_type {
            PacketType::TestOne => {
                if let Ok(payload) = serde_json::from_value::<PayloadOne>(
                    serde_json::to_value(packet.packet_payload).unwrap(),
                ) {
                    PayloadOneHandler::process(payload);
                }
            }
            PacketType::TestTwo => {
                if let Ok(payload) = serde_json::from_value::<PayloadTwo>(
                    serde_json::to_value(packet.packet_payload).unwrap(),
                ) {
                    PayloadTwoHandler::process(payload);
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PayloadOne {
    pub one: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PayloadTwo {
    pub one: String,
}

pub struct PayloadOneHandler {}
impl PayloadOneHandler{
    pub fn process(p: PayloadOne) {
        println!("PayloadOneHandler process")
    }
}

pub struct PayloadTwoHandler {}
impl PayloadTwoHandler{
    pub fn process(p: PayloadTwo) {
        println!("PayloadTwoHandler process")
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::definition::packet_request::{AsyncPacketProcessor, PacketRequest, PacketType, PayloadOne, PayloadTwo};

    #[test]
    fn test_process() {
        let request_one = PacketRequest {
            packet_type: PacketType::TestOne,
            packet_payload: PayloadOne {
                one: String::from("Payload for TestOne"),
            },
        };

        AsyncPacketProcessor::process(request_one);

        let request_two = PacketRequest {
            packet_type: PacketType::TestTwo,
            packet_payload: PayloadTwo {
                one: String::from("Payload for TestTwo"),
            },
        };

        AsyncPacketProcessor::process(request_two);


    }
}