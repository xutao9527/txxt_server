use crate::protocol::payload::test_one::PayloadTwo;

// 定义 PayloadTwoHandler 处理器
pub struct PayloadTwoHandler;
impl PayloadTwoHandler {
    pub fn process(payload: PayloadTwo) {
        println!("Handling PayloadTwo: {:?}", payload);
    }
}