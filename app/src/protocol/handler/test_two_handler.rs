use crate::protocol::handler::packet_handler::PacketHandler;
use crate::protocol::payload::test_two::TestTwo;

pub struct TestTwoHandler;

impl PacketHandler for TestTwoHandler {
    type Payload = TestTwo;

    fn handle(payload: Self::Payload) {
        println!("Handling T2Req: {:?}", payload);
    }
}