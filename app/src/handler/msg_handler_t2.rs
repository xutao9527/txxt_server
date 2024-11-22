use crate::entity::msg_req::T2Req;
use crate::handler::msg_handler::MsgHandler;

pub struct T2Handler;
impl MsgHandler for T2Handler {
    type Payload = T2Req;

    fn handle(payload: Self::Payload) {
        println!("Handling T2Req: {:?}", payload);
    }
}