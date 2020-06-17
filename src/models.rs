use cf_kv_derive::CloudFlareKV;
use serde::{Deserialize, Serialize};

#[derive(CloudFlareKV, Debug, Serialize, Deserialize)]
pub struct UserScore {
    pub email: String,
    pub score: u8,
}

#[derive(CloudFlareKV, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub name: String,
}
