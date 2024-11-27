use app::App;

mod app;
// mod dealer_client;

#[tokio::main]
async fn main() {
    // let mut client = DealerClient::new("127.0.0.1:8080".to_string());
    // client.open().await?;
    // let login_req = PacketRequest {
    //     packet_type: PacketType::Login,
    //     packet_payload: PacketPayload::LoginReq(LoginReq {
    //         user_name: "101".to_string(),
    //         pass_word: "7735a36e802561ef44249c039c8db410".to_string(),
    //     }),
    // };
    // client.send(login_req).await?;
    // client.receive().await;
    // client.close().await;
    // Ok(())
    let terminal = ratatui::init();
    App::new().run(terminal);
    ratatui::restore();
}
