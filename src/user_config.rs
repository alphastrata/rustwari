#[allow(unused_imports)]
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Read;
// use std::path;
// use std::path::Path;
use std::path::PathBuf;
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tilesdir: String,
    pub completed: String,
    pub tmp: String,
}
impl Config {
    #[allow(dead_code)]
    pub fn new(tilesdir: String, completed: String, tmp: String, _keep_for: u32) -> Self {
        Self {
            tilesdir,
            completed,
            tmp,
        }
    }
    #[allow(dead_code)]
    pub fn new_from_yaml(p: PathBuf) -> Result<Self, std::io::Error> {
        let mut file: std::fs::File = std::fs::File::open(p).expect("Unable to open config file.");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read config file.");
        let config = serde_yaml::from_str(&contents[..]).expect("unable to parse config.yml");

        Ok(config)
    }
}
/// read a user's config to ascertain where to store temporary files/complete discs etc.
#[allow(dead_code)]
pub fn read_user_config() {
    todo!();
}
