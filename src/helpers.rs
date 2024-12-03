use crate::password_struct::PasswordStruct;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
pub const PASSWORD_PATH_STR: &str = "data/file.json";
pub const _AUTH_PATH_STR: &str = "auth/keys.json";

pub fn _check_file_exists(path: &str) -> bool {
    let path = create_path(path);
    path.is_file()
}
/// Writes the contents of a Vector to a file.
/// The contents are usually the encrypted passwords to be saved.
/// ## Parameters
/// - content: `Vec<PasswordStruct>` - the passwords to write to file
/// - path: `&str` - the path to the file to write to.
/// 
pub fn write_to_file(content: Vec<PasswordStruct>, path: &str) {
    let path = create_path(path);
    let file = File::create(path);
    match file {
        Ok(mut file) => {
            let mut items_to_write = String::new();
            for item in content {
                let current_line = serde_json::to_string(&item);
                items_to_write = items_to_write + &current_line.unwrap() + "\n";
            }
            let write_result = file.write_all(items_to_write.as_bytes());
            match write_result {
                Ok(_) => println!("Write finished successfully"),
                Err(_) => println!("Error while writing to file"),
            }
        }
        Err(_) => println!("Error opening file"),
    }
}
/// Reads the contents of a file and converts into a Vector of Strings.
/// ## Parameters
/// - path: `&str` - the path to the file to read
/// 
/// ## Returns
/// `Vec<String>` - the contents of the file if the read was a success
/// 
pub fn read_file(path: &str) -> Vec<String> {
    let path = create_path(path);
    let mut items_vector: Vec<String> = Vec::new();
    let file = File::open(path);
    match file {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line;
                match line {
                    Ok(line_value) => items_vector.push(line_value),
                    Err(_) => println!("Error reading a line"),
                }
            }
        }
        Err(_) => println!("Error opening file to read"),
    }
    items_vector
}

/// Search for a password belonging to a specific site or platform.
/// ## Parameters
/// - password_site: `String` - the site to search for it's password
/// ## Returns 
/// `Option<PasswordStruct>` - If the password exists for provided site, returns `Some<PasswordStruct>`
/// else, return `None`
/// 
pub fn get_password(password_site: String) -> Option<PasswordStruct> {
    let file_contents: Vec<String> = read_file(PASSWORD_PATH_STR);
    let mut deserialized: PasswordStruct;
    for item in file_contents {
        deserialized = serde_json::from_str(&item).unwrap();
        if password_site.as_ref() == deserialized.site.to_string() {
            return Some(deserialized);
        }
    }
    None
}
/// Creates a `Path` from a string slice and returns it.
pub fn create_path(path_string: &str) -> &Path {
    let path = Path::new(path_string);
    path
}
