use serde::{Deserialize, Serialize};
use crate::entity::msg_payload::MsgPayload;
use crate::entity::msg_type::MsgType;


#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct MsgReq {
    pub msg_type: MsgType,
    pub msg_payload: MsgPayload,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct T1Req {
    content: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct T2Req {
    content1: String,
    content2: String,
}

impl From<T1Req> for MsgPayload {
    fn from(t1: T1Req) -> Self {
        MsgPayload::T1Req(t1)
    }
}

impl From<T2Req> for MsgPayload {
    fn from(t2: T2Req) -> Self {
        MsgPayload::T2Req(t2)
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::msg_req::{MsgPayload, MsgReq, MsgType, T1Req};
    use crate::handler::msg_handler_t1::T1Handler;
    use crate::handler::msg_handler_t2::T2Handler;
    use crate::handler::msg_processor::AsyncMsgProcessor;

    #[tokio::test]
    async fn test_json() {
        let mut processor = AsyncMsgProcessor::new();
        processor.register_handler::<T1Handler>(MsgType::T1Req).await;
        processor.register_handler::<T2Handler>(MsgType::T2Req).await;

        let msg = MsgReq {
            msg_type: MsgType::T1Req,
            msg_payload: MsgPayload::T1Req(T1Req {
                content: "Hello, T1".to_string(),
            }),
        };

        processor.process(msg);


        // let json_string = serde_json::to_string(&msg).expect("Failed to serialize to JSON");
        // println!("Serialized JSON: {}", json_string);
        // let deserialized_msg: MsgReq =
        //     serde_json::from_str(&json_string).expect("Failed to deserialize from JSON");
        // println!("Deserialized MsgReq: {:?}", deserialized_msg);
        // match deserialized_msg.msg_type {
        //     MsgType::T1Req => {
        //         let msg_payload = match deserialized_msg.msg_payload {
        //             MsgPayload::T1Req(payload) => {
        //                 println!("handle T1Req {:?}", payload);
        //             }
        //             MsgPayload::T2Req(payload) => {}
        //         };
        //     }
        //     MsgType::T2Req => {
        //         let msg_payload = match deserialized_msg.msg_payload {
        //             MsgPayload::T1Req(_) => {}
        //             MsgPayload::T2Req(payload) => {
        //                 println!("handle T2Req {:?}", payload);
        //             }
        //         };
        //     }
        // }
    }
}
