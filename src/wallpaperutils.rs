use crate::cvutils::get_dims;
use anyhow::Error;
use anyhow::Result;
use image::imageops::FilterType;
use image::ImageFormat;
use std::fs;
use std::path::PathBuf;
use wallpaper::{set_from_path, set_mode};

/// Representing a full, 121MP image.
pub struct FullDisc {
    pub path: PathBuf,
    //TODO: set the screen dims based on a user's machine, then only fetch something close to
    //what'll make sense...
    pub(crate) height: i32,
    pub(crate) width: i32,
    pub(crate) size: u64, // NOTE: a daytime image is almos 150mb, the nightime images can be as little as 20mb.
}

impl FullDisc {
    pub(crate) fn new(p: &PathBuf) -> Result<Self> {
        let (width, height) = get_dims(p)?;
        let size = std::fs::metadata(&p.clone())?.len();

        Ok(Self {
            path: fs::canonicalize(p)?,
            height,
            width,
            size,
        })
    }
    /// Sets the current wallpaper to whatever's stored in the path field.
    pub fn set_this(&self) -> Result<(), Error> {
        let _ = set_from_path(
            self.path
                .to_str()
                .unwrap_or_else(|| panic!("Unable to cast Path as String.")),
        );
        let _ = set_mode(wallpaper::Mode::Fit);
        Ok(())
    }

    /// When called on FullDisc it resizes the 121MP .png to a smaller jpeg
    /// Note: This method replaces the .png file (original) with the resized one.
    pub fn resize_this(&mut self, width: i32, height: i32) -> Result<()> {
        let img = image::open(&self.path)?;
        let resized = img.resize(width as u32, height as u32, FilterType::Lanczos3);

        resized.save_with_format(&self.path, ImageFormat::Jpeg)?;
        self.path = fs::canonicalize(&self.path)?;
        Ok(())
    }
}
