use dealer::protocol::definition::packet_request::PacketRequest;
use dealer::protocol::handler::PacketType;
use dealer::protocol::payload::login::LoginReq;
use dealer::protocol::payload::PacketPayload;
use dealer_client::DealerClient;


mod dealer_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DealerClient::new("127.0.0.1:8080".to_string());
   
    client.open().await?;
   
    let login_req = PacketRequest {
        packet_type: PacketType::Login,
        packet_payload: PacketPayload::LoginReq(LoginReq {
            user_name: "101".to_string(),
            pass_word: "7735a36e802561ef44249c039c8db410".to_string(),
        }),
    };
 
    client.send(login_req).await?;
    
    client.receive().await;

    client.close().await;

    Ok(())
}
