use crate::protocol::connection::client_connection::ClientConnection;
use crate::protocol::definition::packet_request::PacketRequest;
use crate::protocol::handler::login_handler::LoginHandler;
use crate::protocol::handler::PacketType;
use crate::protocol::payload::PacketPayload;

pub async fn process_packet(data: &[u8], connection: &mut ClientConnection) {
    let packet: Result<PacketRequest, serde_json::Error> = serde_json::from_slice(data);
    match packet {
        Ok(packet) => {
            // 根据 PacketType 执行不同的处理
            match packet.packet_type {
                PacketType::Login => {
                    let PacketPayload::LoginReq(payload) = packet.packet_payload;
                    LoginHandler::process(payload, connection).await;
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
    use super::*;
    use crate::protocol::payload::login::LoginReq;
    use tokio_util::bytes::BytesMut;
    use tokio_util::codec::{Encoder, LengthDelimitedCodec};

    #[test]
    fn test_process_packet() {
        //let login_str = r#"{"packet_type":"Login","packet_payload":{"user_name":"test","pass_word":"test"}}"#;
        let login_req = PacketRequest {
            packet_type: PacketType::Login,
            packet_payload: PacketPayload::LoginReq(LoginReq {
                user_name: "101".to_string(),
                pass_word: "7735a36e802561ef44249c039c8db410".to_string(),
            }),
        };
        let login_req_str = serde_json::to_string(&login_req).unwrap().to_string();
        let mut buf = BytesMut::new();
        let mut codec = LengthDelimitedCodec::new();
        let _ = codec.encode(login_req_str.into(), &mut buf);
        let hex_output: String = buf.iter().map(|byte| format!("{:02x}", byte)).collect();
        println!("Encoded in hex: {}", hex_output);
    }
}
