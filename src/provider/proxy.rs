use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Proxy {
    pub ip: String,
    pub scheme: String,
    pub auth: Option<String>,
}
