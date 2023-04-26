use anyhow::{Error, Result};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::fs::write;
use std::io::Read;
use std::path::Path;

//TODO: paths for anyone...
pub const CONFIGDIR: &str = ".";
pub const USERCONFIG: &str = "config.yml";

/// Holds all the information a user will need to populate their yaml file with in order to
/// control how rustwari organises downloaded files etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    //TODO: PathBufs
    pub completed: String,
    pub backup: String,
}

impl Config {
    pub fn new_from_yaml<P>(p: P) -> Result<Self, std::io::Error>
    where
        P: AsRef<Path>,
    {
        //FIXME: unwrap()s
        let mut file: std::fs::File = std::fs::File::open(p).expect("Unable to open config file.");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let config = serde_yaml::from_str(&contents).expect("unable to parse config.yml");

        Ok(config)
    }

    /// Creates a user's default yml, should the not have one etc..
    pub fn check_config_exits<P>(p: P) -> Result<(), Error>
    where
        P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>,
    {
        debug!("check_config_exits trigger");
        if Path::new(&p).is_file() {
            debug!("config.yml config file found!");
        } else {
            warn!("config not found, creating");
            if std::fs::create_dir_all(CONFIGDIR).is_ok() {
                Self::create_yml()?;
            } else {
                panic!("Unable to create a directory in $HOME/.config for rustwari to store its config.");
            }
        }
        Ok(())
    }
    /// Creates the default .yml config file
    fn create_yml() -> Result<(), std::io::Error> {
        //TODO: make a static'
        let defaults = r"
# This is the default config, you should change the paths here such that they suit your needs.
# Where do you want to store completed images?
completed: completed

# Where do you want those stored images backed up to, NOTE: This must be different to the above.
backup: backup";

        write(USERCONFIG, defaults).expect("unable to write user config");
        debug!("user's config.yml written to disk");

        Ok(())
    }
}
