// use std::collections::HashMap;
// use crate::protocol::definition::packet_request::PacketRequest;
// use crate::protocol::handler::packet_handler::PacketHandler;
// use crate::protocol::payload::packet_payload::PacketPayload;
// use crate::protocol::payload::packet_type::PacketType;
//
// type BoxedHandler = Box<dyn Fn(PacketPayload) + Send + Sync>;
//
// pub struct AsyncPacketProcessor {
//     handlers: HashMap<PacketType, BoxedHandler>,
// }
//
// impl AsyncPacketProcessor {
//     pub fn new() -> Self {
//         Self {
//             handlers: HashMap::new(),
//         }
//     }
//
//     pub fn register_handler<H>(&mut self, packet_type: PacketType)
//         where
//             H: PacketHandler,
//             H::Payload: Into<PacketPayload>,
//     {
//         let packet_type_clone = packet_type.clone();
//         let handler = move |payload: PacketPayload| {
//             if let Ok(payload) = serde_json::from_value(serde_json::to_value(payload).unwrap()) {
//                 H::handle(payload);
//             } else {
//                 // 直接使用 msg_type，这时 msg_type 的所有权已经转移到闭包中了
//                 eprintln!("Failed to process payload for {:?}", packet_type_clone);
//             }
//         };
//         self.handlers.insert(packet_type, Box::new(handler));
//     }
//
//     // pub fn process(&self, req: PacketRequest) {
//     //     if let Some(handler) = self.handlers.get(&req.packet_type) {
//     //         handler(req.packet_payload);
//     //     } else {
//     //         eprintln!("No handler found for {:?}", req.packet_type);
//     //     }
//     // }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::protocol::definition::packet_request::PacketRequest;
//     use crate::protocol::handler::test_one_handler::TestOneHandler;
//     use crate::protocol::handler::test_two_handler::TestTwoHandler;
//     use crate::protocol::payload::packet_payload::PacketPayload;
//     use crate::protocol::payload::packet_type::PacketType;
//     use crate::protocol::payload::test_one::TestOne;
//     use crate::protocol::payload::test_two::TestTwo;
//     use crate::protocol::processor::packet_processor::AsyncPacketProcessor;
//
//     #[test]
//     fn test_process() {
//         let pkg_req1 = PacketRequest {
//             packet_type: PacketType::TestOne,
//             packet_payload: PacketPayload::TestOne(TestOne::default()),
//         };
//         let pkg_req2 = PacketRequest {
//             packet_type: PacketType::TestTwo,
//             packet_payload: PacketPayload::TestTwo(TestTwo::default()),
//         };
//         //let pkg_str = serde_json::to_string(&pkg_req).expect("Failed to serialize to JSON");
//         let mut processor = AsyncPacketProcessor::new();
//         processor.register_handler::<TestOneHandler>(PacketType::TestOne);
//         processor.register_handler::<TestTwoHandler>(PacketType::TestTwo);
//
//         processor.process(pkg_req1);
//         processor.process(pkg_req2);
//     }
// }