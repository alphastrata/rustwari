use crate::cvutils::get_dims;

use anyhow::Error;
use anyhow::Result;
use image::imageops::FilterType;
use image::ImageFormat;
use log::debug;
use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "linux")]
/// As a dark-mode user the wallpaper crate doesn't accomidate my paticular gsettings requirements...
pub fn set_from_path(p: PathBuf) -> Result<(), std::io::Error> {
    let fp = format!("{}", &p.display());
    debug!("setting_with arg: {:?} ", fp);

    debug!("Detected Pop!_OS, so running for that.");
    // Set the BG for Pop, when it's in darkmode
    let _set= Command::new("gsettings")
    .arg("set")
    .arg("org.gnome.desktop.background")
    .arg("picture-uri-dark") //NOTE: will break for non dark-mode users.
    .arg(fp)
    .spawn()
    .expect("Unable to set image, potential issue with the hardcoded approach to changing/setting wallpaper?\nOr, it could be that your Pop!_OS's theme is not set to dark-mode.");

    // specialised centering for Pop!
    //gsettings set org.gnome.desktop.background picture-options 'centered'
    let _center = Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.background")
        .arg("picture-options")
        .arg("scaled")
        .spawn()
        .expect("Unable to center image.");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
/// Sets the background for any non pop!_os OS.
pub fn set_from_path(p: PathBuf) {
    wallpaper::set_from_path(p.to_str().unwrap()).unwrap();
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
        if self.path.metadata().ok().unwrap().len() > 0 {
            _ = set_from_path(self.path.clone());
            Ok(())
        } else {
            panic!("File failed to concatenate/parse or something, check your directories are set correctly etc.")
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
