use std::fs::{read_to_string, write};

pub fn store_keypair(name: &str, key: &str) {
    let config_path = dirs::home_dir()
        .map(|p| p.join(".config/.kotarurc"))
        .expect("Could not determine home directory");

    let new_entry = format!("{}={}", name, key);

    let new_content = match read_to_string(&config_path) {
        Ok(data) => format!("{}\n{}", data.trim(), new_entry), // Append keypair
        Err(_) => new_entry, // If file doesn't exist, create a new one
    };

    if let Err(err) = write(&config_path, new_content) {
        println!("Failed to write file: {}", err);
    }
}
