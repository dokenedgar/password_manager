pub struct EncryptionResult {
    pub nonce: aes_gcm::aead::generic_array::GenericArray<
        u8,
        aes_gcm::aes::cipher::typenum::UInt<
            aes_gcm::aes::cipher::typenum::UInt<
                aes_gcm::aes::cipher::typenum::UInt<
                    aes_gcm::aes::cipher::typenum::UInt<
                        aes_gcm::aes::cipher::typenum::UTerm,
                        aes_gcm::aead::consts::B1,
                    >,
                    aes_gcm::aead::consts::B1,
                >,
                aes_gcm::aead::consts::B0,
            >,
            aes_gcm::aead::consts::B0,
        >,
    >,
    pub encrypted_text: Vec<u8>,
    pub key: Vec<u8>,
}
