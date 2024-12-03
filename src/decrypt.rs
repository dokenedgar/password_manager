use crate::helpers;
use crate::utils;
pub fn decrypt(args: Vec<String>) {
    if args.len() != 1 {
        println!("Provide name of site to retrieve password for");
    } else {
        let possible_password = helpers::get_password(args.get(0).unwrap().to_string());
        match possible_password {
            Some(password) => {
                let decrypted_password = utils::decrypt_data(&password);
                match decrypted_password {
                    Ok(value) => println!("Password is: {}", value),
                    Err(e) => println!("Error decrypting password, {}", e),
                }
            }
            None => {
                println!("NO PASSWORD FOUND FOR THIS SITE");
            }
        }
    }
}
