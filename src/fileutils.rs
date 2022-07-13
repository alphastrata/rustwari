use anyhow::Result;
use std::path::Path;
use std::path::PathBuf;

const BACKUP: &str = "/media/jer/ARCHIVE/HIMAWARI_DATA";

// check that the completed and tmp directories exist, and if not create them.
pub(crate) async fn check_setup() -> Result<()> {
    if !exists(Path::new("completed").to_path_buf()).await {
        std::fs::create_dir_all("completed")?;
    }
    if !exists(Path::new("tmp").to_path_buf()).await {
        std::fs::create_dir_all("tmp")?;
    }
    Ok(())
}
// A helper to check filesize and if 0 remove it, makes a count of removed failures, tokiofetcher can use that count to refetch
pub(crate) fn remove_failed_and_rerun() -> Result<u32, std::io::Error> {
    let mut count = 0;
    for entry in std::fs::read_dir("tmp")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().expect("Unable to view file extension.") == "png" {
            let metadata = path.metadata()?;
            if metadata.len() == 0 {
                println!("{} is 0 bytes, removing", path.display());
                std::fs::remove_file(path)?;
                count += 1;
            }
        }
    }
    Ok(count)
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
pub(crate) async fn exists(path: PathBuf) -> bool {
    if tokio::fs::metadata(&path).await.is_ok() {
        return true;
    }
    false
}
// Old tiles, or intermediate concats should be cleaned up
pub(crate) fn cleanup_tmp() -> Result<bool, std::io::Error> {
    for entry in std::fs::read_dir("tmp")? {
        let path = entry?.path();
        if path.extension().expect("Unable to view file extension.") == "png" {
            std::fs::remove_file(path)?;
        }
    }
    for entry in std::fs::read_dir("completed")? {
        let path = entry?.path();
        if !path.to_str().unwrap().contains("fulldisc") {
            std::fs::remove_file(path)?;
        }
    }
    Ok(true)
}
