use bytes::Bytes;
use futures_util::SinkExt;
use crate::protocol::connection::client_connection::ClientConnection;

use crate::protocol::payload::login::{LoginReq, LoginRes};


// 定义 PayloadOneHandler 处理器
pub struct LoginHandler;

impl LoginHandler {
    pub async fn process(_: LoginReq, connection: &mut ClientConnection) {
        // 设置认证成功
        connection.authentication = true;
        // 回写响应数据
        let login_res_str = serde_json::to_string(&LoginRes { result: true }).unwrap();
        let _ = connection.writer.send(Bytes::from(login_res_str)).await;
    }
}