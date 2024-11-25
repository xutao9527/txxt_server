use crate::protocol::definition::packet_payload::PacketPayload;
use crate::protocol::definition::packet_request::PacketRequest;
use crate::protocol::definition::packet_type::PacketType;
use crate::protocol::handler::payload_one_handler::PayloadOneHandler;
use crate::protocol::handler::payload_two_handler::PayloadTwoHandler;

// 模拟处理不同类型的 payload
fn process_packet(packet: PacketRequest) {
    match packet.packet_type {
        PacketType::TestOne => {
            // 反序列化为 PayloadOne 并调用处理器
            if let PacketPayload::TestOne(payload) = packet.packet_payload {
                PayloadOneHandler::process(payload);
            } else {
                eprintln!("Unexpected payload for TestOne");
            }
        }
        PacketType::TestTwo => {
            // 反序列化为 PayloadTwo 并调用处理器
            if let PacketPayload::TestTwo(payload) = packet.packet_payload {
                PayloadTwoHandler::process(payload);
            } else {
                eprintln!("Unexpected payload for TestTwo");
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

        // 将 JSON 字符串解析为 PacketRequest 对象
        let request: PacketRequest = serde_json::from_str(json_str).unwrap();

        // 处理解析后的请求
        process_packet(request);
    }

    #[test]
    fn test_process_packet_test_two() {
        // 假设从网络上接收到的 JSON 字符串
        let json_str = r#"{
            "packet_type": "TestTwo",
            "packet_payload": {"one": "Payload for TestTwo"}
        }"#;

        // 将 JSON 字符串解析为 PacketRequest 对象
        let request: PacketRequest = serde_json::from_str(json_str).unwrap();

        // 处理解析后的请求
        process_packet(request);
    }
}
