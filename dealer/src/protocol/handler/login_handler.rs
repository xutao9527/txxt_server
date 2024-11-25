use crate::protocol::payload::login::LoginReq;


// 定义 PayloadOneHandler 处理器
pub struct LoginHandler;
impl LoginHandler {
    pub fn process(login_req: LoginReq) {
        println!("LoginHandler process: {:?}", login_req);
    }
}