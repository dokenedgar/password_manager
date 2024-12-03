use std::env;
mod decrypt;
mod encrypt;
mod encryption_result;
mod helpers;
mod password_struct;

mod utils;

///
/// To run program, enter the following in terminal:
/// 
/// ```
/// cargo run -- [command] service/website password
/// ```
/// where [command] is either encrypt or decrypt
/// ## Example
/// - to encrypt: `cargo run -- encrypt google.com/example password`
/// - to decrypt: `cargo run -- decrypt google.com/example`
///
fn main() {
    let arguments: Vec<String> = env::args().collect();
    println!("{:?}", arguments);
    let action_argument = arguments.get(1);
    let action = match action_argument {
        Some(action_name) => action_name.to_string(),
        None => "unknown_action".to_string(),
    };
    let args = arguments[2..].to_vec();
    match action.as_str() {
        "encrypt" => {
            encrypt::encrypt(args);
        }
        "decrypt" => {
            let args = arguments[2..].to_vec();
            decrypt::decrypt(args);
        }
        _ => println!("Dunno what we are doing"),
    }
    println!("\n");
}
