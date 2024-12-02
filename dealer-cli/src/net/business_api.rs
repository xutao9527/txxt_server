use dealer::protocol::{definition::packet_request::PacketRequest, handler::PacketType, payload::{login::LoginReq, PacketPayload}};

use crate::app::app_data::{AppData, ConnectInfo};



pub fn login(app_data:&mut AppData, connect_info:ConnectInfo){
    let login_req = PacketRequest {
        packet_type: PacketType::Login,
        packet_payload: PacketPayload::LoginReq(LoginReq {
            user_name: connect_info.table_no,
            pass_word: connect_info.password,
        }),
    };
    let _ = app_data.dealer_client.send(login_req);
    // let _ = data.dealer_client.send(login_req);
}