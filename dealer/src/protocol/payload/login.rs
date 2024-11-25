use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LoginReq {
    pub user_name: String,
    pub pass_word: String,
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LoginRes {
    pub result: bool,
}