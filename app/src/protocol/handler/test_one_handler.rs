use crate::protocol::handler::packet_handler::PacketHandler;
use crate::protocol::payload::test_one::TestOne;

pub struct TestOneHandler;

impl PacketHandler for TestOneHandler {
    type Payload = TestOne;

    fn handle(payload: Self::Payload) {
        println!("Handling T2Req: {:?}", payload);
    }
}

