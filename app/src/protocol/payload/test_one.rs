use serde::{Deserialize, Serialize};

// 定义 PayloadOne 结构体
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PayloadOne {
    pub one: String,
}

// 定义 PayloadTwo 结构体
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PayloadTwo {
    pub one: String,
}
