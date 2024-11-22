use crate::entity::msg_req::T1Req;
use crate::handler::msg_handler::MsgHandler;

pub struct T1Handler;
impl MsgHandler for T1Handler {
    type Payload = T1Req;

    fn handle(payload: Self::Payload) {
        println!("Handling T1Req: {:?}", payload);
    }
}