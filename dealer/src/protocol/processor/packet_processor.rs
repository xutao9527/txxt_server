use crate::protocol::definition::packet_request::PacketRequest;
use crate::protocol::handler::login_handler::LoginHandler;
use crate::protocol::handler::PacketType;
use crate::protocol::payload::PacketPayload;


// 模拟处理不同类型的 payload
fn process_packet(packet: PacketRequest) {
    match packet.packet_type {
        PacketType::Login => {
            if let PacketPayload::LoginReq(payload) = packet.packet_payload {
                LoginHandler::process(payload);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::payload::login::LoginReq;
    use super::*;
    #[test]
    fn test_process_packet() {
        let login_req =  PacketRequest{
            packet_type:PacketType::Login,
            packet_payload:PacketPayload::LoginReq(LoginReq{
                user_name:"test".to_string(),
                pass_word:"test".to_string(),
            })
        };

        let login_str = serde_json::to_string(&login_req).unwrap();
        println!("{}",login_str);

        let login_str = r#"{"packet_type":"Login","packet_payload":{"user_name":"test","pass_word":"test"}}"#;

        let request: PacketRequest = serde_json::from_str(login_str).unwrap();
        process_packet(request);

    }
}
