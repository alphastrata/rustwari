use crate::cvutils::get_dims;

use anyhow::Error;
use anyhow::Result;
use image::imageops::FilterType;
use image::ImageFormat;
use log::debug;
use std::fs;
use std::path::PathBuf;

/// Sets the background for any non pop!_os OS.
pub fn set_from_path<P: Into<String>>(p: P) {
    wallpaper::set_from_path(&p.into()).expect("Unable to set wallpaper.");
}

#[derive(Debug)]
/// Representing a full, 121MP [`image`]
pub struct FullDisc {
    pub path: PathBuf,
    //TODO: set the screen dims based on a user's machine, then only fetch something close to
    //what'll make sense...
    pub(crate) height: u32,
    pub(crate) width: u32,
    pub(crate) size: u64, // NOTE: a daytime image is almos 150mb, the nightime images can be as little as 20mb.
}

impl FullDisc {
    pub(crate) fn new(p: &PathBuf) -> Result<Self> {
        let (width, height) = get_dims(p)?;
        let size = std::fs::metadata(p.clone())?.len();

        Ok(Self {
            path: fs::canonicalize(p)?,
            height,
            width,
            size,
        })
    }
    /// Sets the current wallpaper to whatever's stored in the path field.
    pub fn set_this(&self) -> Result<(), Error> {
        //TODO: fix these unwraps..
        if self.path.metadata().ok().unwrap().len() > 0 {
            _ = set_from_path::<String>(self.path.to_str().unwrap().into());
            Ok(())
        } else {
            panic!("File failed to concatenate/parse or something, check your directories are set correctly etc.")
        }
    }

    /// Sets the wallpaper wit hyperland's hyprpaper (wayland only)
    #[cfg(feature = "hypr")]
    pub fn set_with_hyprpaper(&self) {
        // Unload
        let hyprctrl_unload = std::process::Command::new("hyprctl")
            .arg("hyprpaper")
            .arg("unload")
            .arg("all")
            .output()
            .expect("Failed to execute command");

        if !hyprctrl_unload.status.success() {
            eprintln!(
                "hyprctrl unload failed with: {}",
                String::from_utf8_lossy(&hyprctrl_unload.stderr)
            );
        }

        // Preload
        let hyprctrl_preload = std::process::Command::new("hyprctl")
            .arg("hyprpaper")
            .arg("preload")
            .arg(format!("{}", self.path.display()))
            .output()
            .expect("Failed to execute command");
        log::debug!("Preloading: {:?}", self.path.display());

        if !hyprctrl_preload.status.success() {
            eprintln!(
                "hyprctrl preload failed with: {}",
                String::from_utf8_lossy(&hyprctrl_preload.stderr)
            );
        }

        // Set
        let hyprctrl_set = std::process::Command::new("hyprctl")
            .arg("hyprpaper")
            .arg("wallpaper")
            .arg(format!("DP-1,contain:{}", self.path.display()))
            .output()
            .expect("Failed to execute command");

        if !hyprctrl_set.status.success() {
            eprintln!(
                "hyprctrl wallpaper failed with: {}",
                String::from_utf8_lossy(&hyprctrl_set.stderr)
            );
        }
    }

    /// When called on [`FullDisc`] it resizes the 121MP .png to a smaller jpeg
    /// Note: This method replaces the .png file (original) with the resized one.
    pub fn resize_this(&mut self, width: u32, height: u32) -> Result<()> {
        let img = image::open(&self.path)?;
        let resized = img.resize(width, height, FilterType::Lanczos3);

        resized.save_with_format(&self.path, ImageFormat::Jpeg)?;
        self.path = fs::canonicalize(&self.path)?;
        debug!("Resize, success: {}", &self.path.display());
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[ignore]
    #[test]
    fn set_from_path_works() {
        let path = PathBuf::from("test_data/fulldisc-2022-09-21 00_10.png");
        _ = set_from_path(path);
    }

    #[ignore]
    #[test]
    fn os_info() {
        let info = os_info::get();

        // Print full information:
        println!("OS information: {info}");

        // Print information separately:
        println!("Type: {}", info.os_type()); //NOTE: this is what we'll use
        println!("Version: {}", info.version());
        println!("Bitness: {}", info.bitness());
    }
}
