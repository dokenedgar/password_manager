use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyStruct {
    pub key: Vec<u8>,
    pub site: String
}