use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct PasswordStruct {
    pub site: String,
    pub password: Vec<u8>,
    pub nonce: Vec<u8>,
    pub key: Vec<u8>,
}
