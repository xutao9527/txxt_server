use std::collections::HashMap;
use crate::protocol::definition::packet_request::PacketRequest;
use crate::protocol::payload::packet_type::PacketType;

pub struct AsyncPacketProcessor {
    handlers: HashMap<PacketType, String>,
}

impl AsyncPacketProcessor {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register_handler(&mut self,packet_type: PacketType) {
        self.handlers.insert(packet_type, "1".to_string());
    }

    pub fn process(&self, req: PacketRequest) {
        println!("{:#?}",req);
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::definition::packet_request::PacketRequest;
    use crate::protocol::payload::packet_payload::PacketPayload;
    use crate::protocol::payload::packet_type::PacketType;
    use crate::protocol::processor::packet_processor::AsyncPacketProcessor;

    #[test]
    fn test_process() {
        let pkg_req1 = PacketRequest {
            packet_type: PacketType::TestOne,
            packet_payload:PacketPayload::TestOne
        };
        let pkg_req2 = PacketRequest {
            packet_type: PacketType::TestTwo,
            packet_payload:PacketPayload::TestTwo
        };
        //let pkg_str = serde_json::to_string(&pkg_req).expect("Failed to serialize to JSON");
        let mut processor = AsyncPacketProcessor::new();
        processor.register_handler(PacketType::TestOne);
        processor.register_handler(PacketType::TestTwo);

        processor.process(pkg_req1);
        processor.process(pkg_req2);
    }
}