use crate::encryption_result;
use crate::password_struct::PasswordStruct;
use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use std::fmt;

#[derive(Debug)]
struct EncryptionError(aes_gcm::Error);

// Implement Display for our custom error
impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Encryption error: {}", self.0)
    }
}
// Implement std::error::Error for our custom error
impl std::error::Error for EncryptionError {}

/// Use an encryption algorithm to encrypt piece of text
pub fn encrypt_data(
    text: String,
) -> Result<encryption_result::EncryptionResult, Box<dyn std::error::Error>> {
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);

    let message = text;
    let nonce = Aes256Gcm::generate_nonce(OsRng);
    let cipher_text = cipher
        .encrypt(&nonce, message.as_ref())
        .map_err(EncryptionError)?;
    let encrypt_object: encryption_result::EncryptionResult = encryption_result::EncryptionResult {
        nonce: nonce,
        encrypted_text: cipher_text,
        key: key.to_vec(),
    };
    Ok(encrypt_object)
}

/// Decrypts a password by using the `nonce`, and `key` used in initially encrypting that password.
/// If these differ, results in an error. Else returns the string representation of the encrypted data.
pub fn decrypt_data(password: &PasswordStruct) -> Result<String, Box<dyn std::error::Error>> {
    let key = Key::<Aes256Gcm>::from_slice(&password.key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(&password.nonce);
    let decrypt = cipher
        .decrypt(&nonce, password.password.as_ref())
        .map_err(EncryptionError)?;
    let val = String::from_utf8_lossy(&decrypt).to_string();
    println!("Decrypted: {:?}", String::from_utf8_lossy(&decrypt));
    Ok(val)
}
