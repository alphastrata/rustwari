use anyhow::{Error, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::Path;

use std::fs::write;

pub const USERCONFIG: &str = "config.yml";

/// Config holds all the information a user will need to populate their yaml file with in order to
/// control how rustwari organises downloaded files etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tilesdir: String,
    pub completed: String,
    pub tmp: String,
}

impl Config {
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
    pub fn check_config_exits<P>(p: P) -> Result<(), Error>
    where
        P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>,
    {
        info!("check_config_exits trigger");
        if Path::new(&p).is_file() {
            info!("user.yml config file found!");
        }
        info!("config not found, creating");
        Self::create_yml()?;
        Ok(())
    }
    /// Creates the default .yml config file
    fn create_yml() -> Result<(), std::io::Error> {
        let defaults = r"#
                        tilesdir: /media/jer/ARCHIVE/HIMAWARI_DATA/tiles/
                        tmp: /media/jer/ARCHIVE/HIMAWARI_DATA/tmp/
                        completed: /media/jer/ARCHIVE/HIWARI_DATA/completed/#";

        write(USERCONFIG, defaults)?;
        info!("user's config.yml written to disk");

        Ok(())
    }
}
