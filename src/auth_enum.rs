use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub enum AuthEnum {
    PasswordStruct {
        site: String,
        password: Vec<u8>,
        nonce: Vec<u8>,
        
    },
    KeyStruct {
        key: Vec<u8>,
        site: String,
    },
}
