use crate::protocol::definition::packet_request::PacketRequest;
use crate::protocol::handler::login_handler::LoginHandler;
use crate::protocol::handler::PacketType;
use crate::protocol::payload::PacketPayload;



pub async fn process_packet(data: &[u8]) {
    let packet: Result<PacketRequest, serde_json::Error> = serde_json::from_slice(data);
    match packet {
        Ok(packet) => {
            // 根据 PacketType 执行不同的处理
            match packet.packet_type {
                PacketType::Login => {
                    if let PacketPayload::LoginReq(payload) = packet.packet_payload {
                        LoginHandler::process(payload); // 调用 LoginHandler 处理
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to parse packet: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::payload::login::LoginReq;
    use super::*;

    #[test]
    fn test_process_packet() {
        let login_req = PacketRequest {
            packet_type: PacketType::Login,
            packet_payload: PacketPayload::LoginReq(LoginReq {
                user_name: "test".to_string(),
                pass_word: "test".to_string(),
            }),
        };

        let login_str = serde_json::to_string(&login_req).unwrap();
        let login_str = r#"{"packet_type":"Login","packet_payload":{"user_name":"test","pass_word":"test"}}"#;
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            process_packet(login_str.as_bytes()).await;
        });
    }
}
