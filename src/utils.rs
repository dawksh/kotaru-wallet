use dirs;
use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .map(|p| p.join(".config/.kotarurc"))
        .expect("Could not determine home directory")
}
