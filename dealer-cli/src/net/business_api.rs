pub mod req_api {
    use std::error::Error;

    use crate::app::app_data::AppData;
    use dealer::protocol::{
        definition::packet_request::PacketRequest,
        handler::PacketType,
        payload::{login::LoginReq, PacketPayload},
    };

    pub fn open() -> Result<(), Box<dyn Error>> {
        let app_data = AppData::singleton();
        if let Ok(mut data) = app_data.write() {
            let _ = data.client.open()?;
        };
        Ok(())
    }

    pub fn login() {
        let app_data = AppData::singleton();
        if let Ok(mut data) = app_data.write() {
            let select_id = data.client_select_id;
            let connect_info = &data.connects[select_id];
            let login_req = PacketRequest {
                packet_type: PacketType::Login,
                packet_payload: PacketPayload::LoginReq(LoginReq {
                    user_name: connect_info.table_no.clone(),
                    pass_word: connect_info.password.clone(),
                }),
            };
            if let Ok(_) = data.client.send(login_req) {
                data.client_connected_id = Some(select_id);
            }
        };
    }
}
