use crate::protocol::payload::login::LoginReq;


// 定义 PayloadOneHandler 处理器
pub struct LoginHandler;
impl LoginHandler {
    pub fn process(payload: LoginReq) {
        println!("LoginHandler process: {:?}", payload);
    }
}