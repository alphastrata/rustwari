use crate::user_config::Config;
use anyhow::Error;
use anyhow::Result;
use log::info;
use std::path::Path;
use std::path::PathBuf;

//TODO: add backup as a yml option where it belongs
const BACKUP: &str = "/media/jer/ARCHIVE/HIMAWARI_DATA";

// check that the completed and tmp directories exist, and if not create them.
pub async fn check_setup(uc: &Config) -> Result<()> {
    if !exists(Path::new(&uc.completed).to_path_buf()).await {
        std::fs::create_dir_all(&uc.completed)?;
    }
    if !exists(Path::new(&uc.tmp).to_path_buf()).await {
        std::fs::create_dir_all(&uc.completed)?;
    }
    Ok(())
}
// A helper to check filesize and if 0 remove it, makes a count of removed failures, tokiofetcher can use that count to refetch
pub fn remove_failed_and_rerun(uc: &Config) -> Result<u32, Error> {
    let dir = std::fs::read_dir(&uc.tmp)?;
    Ok(dir
        .into_iter()
        .filter_map(|e| {
            let p = e.ok()?.path();
            if p.extension()
                .expect("Expected to be able to read file's extension.")
                == "png"
                && p.metadata().ok()?.len() == 0
            {
                info!("{} is 0 bytes, removing", p.display());
                std::fs::remove_file(p).ok()?;
                Some(1)
            } else {
                None
            }
        })
        .sum::<u32>())
}

pub fn move_completed_to_backup(path: &PathBuf) -> Result<()> {
    // move the conetents of completed to /media/jer/ARCHIVE/HIMAWARI_DATA/
    std::fs::copy(
        path,
        Path::new(BACKUP).join(path.file_name().expect("Unable to view file name.")),
    )?;
    Ok(())
}
/// Returns true if the file/directory exists
pub async fn exists(path: PathBuf) -> bool {
    if tokio::fs::metadata(&path).await.is_ok() {
        return true;
    }
    false
}
// Old tiles, or intermediate concats should be cleaned up
pub fn cleanup_tmp(uc: &crate::user_config::Config) -> Result<bool, std::io::Error> {
    //TODO: convert to filtermap
    for entry in std::fs::read_dir(&uc.tmp)? {
        let path = entry?.path();
        if path.extension().expect("Unable to view file extension.") == "png" {
            std::fs::remove_file(path)?;
        }
    }
    std::fs::read_dir(&uc.tmp)?.into_iter().for_each(|path| {
        let pb = path.expect("unable to make PathBuf from DirEntry").path();
        if pb
            .extension()
            .unwrap_or_else(|| panic!("failed to get extension from file:{:#?}", pb))
            == "png"
        {
            let _ = std::fs::remove_file(pb);
        }
    });

    Ok(true)
}
