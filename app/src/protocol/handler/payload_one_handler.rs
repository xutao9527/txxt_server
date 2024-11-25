use crate::protocol::payload::test_one::PayloadOne;

// 定义 PayloadOneHandler 处理器
pub struct PayloadOneHandler;
impl PayloadOneHandler {
    pub fn process(payload: PayloadOne) {
        println!("Handling PayloadOne: {:?}", payload);
    }
}