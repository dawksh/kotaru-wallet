use std::fs::{read_to_string, write};

pub fn store_keypair(name: &str, key: &str) {
    let config_path = dirs::home_dir()
        .map(|p| p.join(".config/.kotarurc"))
        .expect("Could not determine home directory");

    if !config_path.exists() {
        write(config_path, format!("{}={}", name, key)).unwrap();
        return;
    }

    match read_to_string(&config_path) {
        Ok(data) => println!("{}", data),
        Err(err) => println!("Failed to read file: {}", err),
    }
}
