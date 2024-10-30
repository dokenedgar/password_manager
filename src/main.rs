use std::env;
// cargo run -- [decrypt][encrypt] service/website password
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let action_argument = arguments.get(1);
    let action = match action_argument {
        Some(action_name) => action_name.to_string(),
        None => "unknown_action".to_string(),
    };
    match action.as_str() {
        "encrypt" => {
            println!("We are encrypting")
        }
        "decrypt" => println!("We are decrypting"),
        _ => println!("Dunno what we are doing"),
    }
}
