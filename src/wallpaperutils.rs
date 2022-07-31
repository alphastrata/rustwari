use crate::cvutils::get_dims;
use anyhow::Result;
use image::imageops::FilterType;
use image::ImageFormat;
use std::fs;
use std::path::PathBuf;
use wallpaper::{get, set_from_path, set_mode};
use log::info;

pub(crate) struct FullDisc {
    pub path: PathBuf,
    //TODO: use these to intuitively setup a size appropriate for resizing (if indeed you decide that it's worthwhile to do so..)
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
    pub(crate) fn set_this(&self) -> Result<()> {
        info!("Currently:{:#?}", get().unwrap());
        let _ = set_from_path(self.path.to_str().unwrap());
        let _ = set_mode(wallpaper::Mode::Fit);
        info!("Newly Set:{:#?}", get().unwrap());
        Ok(())
    }

    /// When called on FullDisc it resizes the 121MP .png
    /// Note: This method replaces the .png file (original) with the resized one.
    pub(crate) fn resize_this(&mut self, width: i32, height: i32) -> Result<()> {
        let img = image::open(&self.path)?;
        let resized = img.resize(width as u32, height as u32, FilterType::Lanczos3);
        let p = self.path.to_str().unwrap(); //.replace(".png", ".jpg");
                                             //;// let _ = resized.save(&p);
        resized.save_with_format(&p, ImageFormat::Png)?;
        let p: PathBuf = PathBuf::from(p);
        self.path = fs::canonicalize(p)?;
        Ok(())
    }
}
