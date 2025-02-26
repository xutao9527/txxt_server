use crate::protocol::connection::client_connection::ClientConnection;
use crate::protocol::payload::login::{LoginReq, LoginRes};
use bytes::Bytes;
use dbs::dao::TableBoardDao;
use dbs::model::table_board;
use futures_util::SinkExt;
use sea_orm::{ColumnTrait, Condition};

// 定义 PayloadOneHandler 处理器
pub struct LoginHandler;

impl LoginHandler {
    pub async fn process(login_req: LoginReq, connection: &mut ClientConnection) {
        let condition = Condition::all()
            .add(table_board::Column::TableNo.eq(login_req.user_name))
            .add(table_board::Column::Password.eq(login_req.pass_word));
        match TableBoardDao::find_one(condition).await {
            Ok(Some(_)) => {
                connection.authentication = true;
                let _ = connection
                    .writer
                    .send(Bytes::from(
                        serde_json::to_string(&LoginRes { result: true }).unwrap(),
                    ))
                    .await;
            }
            Ok(_) => {
                let _ = connection
                    .writer
                    .send(Bytes::from(
                        serde_json::to_string(&LoginRes { result: false }).unwrap(),
                    ))
                    .await;
            }
            Err(_) => {}
        }
    }
}
