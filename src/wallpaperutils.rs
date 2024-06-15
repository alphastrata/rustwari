use crate::cvutils::get_dims;

#[cfg(feature = "wpaperd")]
mod wpaperd_utils;
#[cfg(feature = "wpaperd")]
use wpaperd_utils::get_monitor_names;

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

/// Representing a full, 121MP [`image`]
#[derive(Debug)]
#[allow(dead_code)]
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
        #[cfg(feature = "wpaperd")]
        {
            log::debug!("Setting with wlpaperctl");
            self.set_with_wpaperd();
        }

        #[cfg(feature = "hyprpaper")]
        {
            log::debug!("Setting with hyprctl");
            self.set_with_hyprpaper();
        }

        #[cfg(not(any(feature = "wpaperd", feature = "hyprpaper")))]
        {
            if self.path.metadata()?.len() > 0 {
                _ = set_from_path::<String>(self.path.to_str().unwrap().into());
            } else {
                panic!("File failed to concatenate/parse or something, check your directories are set correctly etc.")
            }
        }

        Ok(())
    }

    #[cfg(feature = "wpaperd")]
    pub fn set_with_wpaperd(&self) {
        get_monitor_names().into_iter().for_each(|mon| {
            log::trace!("Attempting to set {} to {}", self.path.display(), mon);
            assert!(self.path.exists());

            // let output = std::process::Command::new("wpaperctl");
            let mut output = std::process::Command::new("nixGL");
            output
                .arg("/home/jer/Documents/wpaperd/target/release/wpaperctl")
                .arg("set")
                .arg(&self.path)
                .arg(mon);

            log::trace!("cmd: {:#?}", output);

            let output = output.output().expect("Failed to execute command");

            if output.status.success() {
                log::debug!("Command executed successfully");
            } else {
                log::error!("Command failed to execute");
                log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        });
    }

    /// Sets the wallpaper wit hyperland's hyprpaper (wayland only)
    #[cfg(feature = "hyprpaper")]
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
