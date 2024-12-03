use crate::encryption_result::EncryptionResult;
use crate::helpers::{self, PASSWORD_PATH_STR};
use crate::password_struct::PasswordStruct;
use crate::utils::encrypt_data;

/// Initiates the process of encrypting a password. 
/// ## Parameters
/// ```
/// Vec<String>
/// ```
/// 
/// The vector has to be of length = 2, where 
/// - the first item is the site of platform that the password is for. 
/// Eg facebook.com/username or anything really, to identify the site
/// - the second item is the actual password to encrypt
/// 
/// Once password is encrypted, it gets written to file.
/// If the site already exists, the password gets updated.
/// 
pub fn encrypt(args: Vec<String>) {
    if args.len() != 2 {
        println!("Invalid inputs.\n
        To encrypt a password, provide a service and the password after the command to start the program.
        \nEG: cargo run -- encrypt facebook.com a_facebook_password\n")
    } else {
        let default: String = "none".to_string();
        let service: &String = args.get(0).unwrap_or(&default);
        let original_password: &String = args.get(1).unwrap_or(&default);

        let password_file_contents: Vec<String> = helpers::read_file(PASSWORD_PATH_STR);
        let mut password_objects: Vec<PasswordStruct> = Vec::new();
        let mut password_updated: bool = false;

        for item in password_file_contents {
            let mut deserialized: PasswordStruct = serde_json::from_str(&item).unwrap();
            if deserialized.site.as_ref() == service.to_string() {
                let result = encrypt_string(original_password);
                deserialized.password = result.encrypted_text;
                deserialized.nonce = result.nonce.to_vec();
                deserialized.key = result.key;
                password_updated = true;
            }
            password_objects.push(deserialized);
        }
        // read file
        // file service exists, update password
        if !password_updated {
            let result = encrypt_string(original_password);
            let new_password: PasswordStruct = PasswordStruct {
                site: service.to_string(),
                password: result.encrypted_text,
                nonce: result.nonce.to_vec(),
                key: result.key,
            };
            password_objects.push(new_password);
        }
        helpers::write_to_file(password_objects, PASSWORD_PATH_STR);
    }
}

/// Wrapper for encrypting a string.
/// ## Parameter
/// ```
/// str: &str
/// ```
/// ## Returns
/// ```
/// EncryptionResult
/// ```
/// - which contains the `encrypted text`,`nonce`, and `key` used for the encryption.
/// These are useful when decrypting back to get the original string
/// 
fn encrypt_string(str: &str) -> EncryptionResult {
    let result = encrypt_data(str.to_string());
    result.unwrap()
}
