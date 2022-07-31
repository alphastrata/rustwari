#[allow(unused_imports)]
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::Path;

use std::fs::{create_dir, write};

pub const USERCONFIG: &str = "config.yml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tilesdir: String,
    pub completed: String,
    pub tmp: String,
}
impl Config {
    pub fn new(tilesdir: String, completed: String, tmp: String, _keep_for: u32) -> Self {
        Self {
            tilesdir,
            completed,
            tmp,
        }
    }
    pub fn new_from_yaml<P>(p: P) -> Result<Self, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let mut file: std::fs::File = std::fs::File::open(p).expect("Unable to open config file.");
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = serde_yaml::from_str(&contents[..]).expect("unable to parse config.yml");

        Ok(config)
    }

    /// creates a user's default yml, should the not have one etc..
    pub fn check_config_exits() -> Result<bool, Error> {
        println!("check_config_exits trigger");
        if Path::new(USERCONFIG).is_file() {
            println!("user.yml config file found!");
            return Ok(true);
        }
        if Path::new("/home/jer/.config/stainglass/").is_dir() {
            Self::create_yml()?;
            println!("Directory exists, but user.yml does not, creating...");
            Ok(true)
        } else {
            create_dir("/home/jer/.config/stainglass/")?;
            Self::create_yml()?;
            println!("Directory does not exist, creating it and user config...");
            Ok(true)
        }
    }
    /// Creates the default .yml config file
    fn create_yml() -> Result<(), std::io::Error> {
        let defaults = r"#
                ./config.yml: line 1: tilesdir:: command not found
                ./config.yml: line 2: tmp:: command not found
                ./config.yml: line 3: completed:: command not found#";

        write(USERCONFIG, defaults)?;
        println!("user's config.yml written to disk");

        Ok(())
    }
}
