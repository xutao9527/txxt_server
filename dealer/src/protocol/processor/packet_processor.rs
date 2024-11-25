use crate::protocol::definition::packet_request::PacketRequest;
use crate::protocol::handler::login_handler::LoginHandler;
use crate::protocol::handler::PacketType;
use crate::protocol::payload::PacketPayload;


// 模拟处理不同类型的 payload
fn process_packet(packet: PacketRequest) {
    match packet.packet_type {
        PacketType::LOGIN => {
            if let PacketPayload::Login(payload) = packet.packet_payload {
                LoginHandler::process(payload);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_packet() {
        // 假设从网络上接收到的 JSON 字符串
        let json_str = r#"{
            "packet_type": "TestOne",
            "packet_payload": {"one": "Payload for TestOne"}
        }"#;
        let request: PacketRequest = serde_json::from_str(json_str).unwrap();
        process_packet(request);
    }
}
