use crate::user_config::Config;

use anyhow::Result;
use log::debug;
use log::warn;
use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
use std::process::Command;

/// check that the completed and tmp directories exist, and if not create them.
pub async fn check_setup(uc: &Config) -> Result<()> {
    // check and set a high ulimit:
    #[cfg(target_os = "macos")]
    increase_ulimit();

    debug!("{:#?}", &uc);
    if !exists(Path::new(&uc.completed).to_path_buf()).await {
        warn!("dir to store completed files does not exist, creating.");
        std::fs::create_dir_all(&uc.completed).unwrap();
    }
    if !exists(Path::new(&uc.backup).to_path_buf()).await {
        warn!("dir to store backup files does not exist, creating.");
        std::fs::create_dir_all(&uc.completed).unwrap();
    }
    debug!(".yaml is good!");

    Ok(())
}

/// Move the conetents of completed to the location specifed in the config.yml
pub fn move_completed_to_backup(path: PathBuf, uc: &Config) -> Result<()> {
    std::fs::copy(
        &path,
        Path::new(&uc.backup).join(path.file_name().expect("Unable to view file name.")),
    )?;
    debug!("Files backed up!");
    Ok(())
}
pub async fn exists(path: PathBuf) -> bool {
    tokio::fs::metadata(&path).await.is_ok()
}
pub fn cleanup(uc: &crate::user_config::Config) -> Result<(), std::io::Error> {
    debug!("Cleanup requested!");

    std::fs::read_dir(&uc.completed)?.for_each(|path| {
        let pb = path.expect("unable to make PathBuf from DirEntry").path();
        if pb
            .extension()
            .unwrap_or_else(|| panic!("failed to get extension from file:{pb:#?}"))
            == "png"
        {
            _ = std::fs::remove_file(pb);
        }
    });
    Ok(())
}

#[cfg(target_os = "macos")]
fn increase_ulimit() {
    Command::new("ulimit")
        .arg("-n")
        .arg("10240")
        .status()
        .expect("Failed to increase ulimit");
}
