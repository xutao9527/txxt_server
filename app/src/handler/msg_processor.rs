use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::entity::msg_payload::MsgPayload;
use crate::entity::msg_req::MsgReq;
use crate::entity::msg_type::MsgType;

use crate::handler::msg_handler::MsgHandler;

type BoxedHandler = Box<dyn Fn(MsgPayload) + Send + Sync>;

pub struct AsyncMsgProcessor {
    handlers: Arc<RwLock<HashMap<MsgType, BoxedHandler>>>,
}

impl AsyncMsgProcessor {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_handler<H>(&mut self, msg_type: MsgType)
        where
            H: MsgHandler + 'static,
            H::Payload: Into<MsgPayload>,
    {
        let msg_type_clone = msg_type.clone();
        let handler = move |payload: MsgPayload| {
            if let Ok(payload) = serde_json::from_value(serde_json::to_value(payload).unwrap()) {
                H::handle(payload);
            } else {
                // 直接使用 msg_type，这时 msg_type 的所有权已经转移到闭包中了
                eprintln!("Failed to process payload for {:?}", msg_type_clone);
            }
        };
        // msg_type 的所有权已经转移到闭包，因此可以继续使用
        self.handlers
            .write()
            .unwrap()
            .insert(msg_type, Box::new(handler));  // 这里 msg_type 已经不再是原来的参数，而是被闭包持有
    }


    pub fn process(&self, msg_req: MsgReq) {
        let handlers = self.handlers.read().unwrap();
        if let Some(handler) = handlers.get(&msg_req.msg_type) {
            handler(msg_req.msg_payload);
        } else {
            eprintln!("No handler found for {:?}", msg_req.msg_type);
        }
    }
}