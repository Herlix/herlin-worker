use kv_derive::Documentize;
use serde::{Deserialize, Serialize};

#[derive(Documentize, Serialize, Deserialize)]
pub struct UserScore {
    pub email: String,
    pub score: u8,
}

#[derive(Documentize, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub name: String,
}
