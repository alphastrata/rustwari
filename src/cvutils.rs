use crate::cli::Cli;
use crate::himawaridt::HimawariDatetime;
use crate::tiles::{img_from, RemoteTile};
use crate::user_config::Config;
use crate::wallpaperutils::FullDisc;

use anyhow::Result;
use bytes::Bytes;
use image::{GenericImage, ImageBuffer};
use kdam::term::Colorizer;
use kdam::{tqdm, BarExt};
use log::{debug, error};
use std::path::Path;
use tokio::sync::mpsc::Receiver;

pub const ROWMAX: u32 = 20;
pub const COLMAX: u32 = 20;
pub const TILE_WIDTH: u32 = 550;

/// Helper to build the entire disk, it uses the row/col numbers
/// from the HashMap in which the LocalTiles are stored to place them into a buffer.
pub async fn assemble_full_disc(
    hwdt: HimawariDatetime,
    uc: &Config,
    cli: &Cli,
    mut rx: Receiver<(Bytes, RemoteTile)>,
) -> Result<FullDisc> {
    debug!("Assembling full disc...");

    // I like to see progress bars.
    let mut pb = tqdm!(
        0..400,
        bar_format = format!(
            "Progress: {{animation}} {}",
            "{percentage:3.0}%".colorize("#EE6FF8")
        ),
        colour = "gradient(#5A56E0,#EE6FF8)",
        force_refresh = true
    );

    // The final image that we're making
    let mut fulldisc = ImageBuffer::new(ROWMAX * TILE_WIDTH, COLMAX * TILE_WIDTH);

    //NOTE: .par_iter() was tested extensively and showed no appreciable benefits (even on my 32 core system.)
    while let Some((bytes, rt)) = rx.recv().await {
        let x_offset = rt.x * TILE_WIDTH;
        let y_offset = rt.y * TILE_WIDTH;

        let img = img_from(bytes);

        match fulldisc.copy_from(&img, x_offset, y_offset) {
            Ok(_) => {
                pb.update(1);
            }
            Err(e) => error!("{e} on x{}, y{}", rt.x, rt.y),
        };
    }

    pb.completed();
    eprintln!(); // Creating space for the progressbars.

    let p = std::path::Path::new(&uc.completed).join(hwdt.pretty_filename());

    fulldisc
        .save(&p)
        .unwrap_or_else(|_| panic!("Error saving: {}", p.display()));

    if cli.verbose {
        debug!("IMAGE:{}", p.display());
    }

    FullDisc::new(&p)
}

/// Get the dimensions of an image from the file path.
pub(crate) fn get_dims<P: AsRef<Path>>(p: P) -> Result<(u32, u32)> {
    let img = image::open(p)?;
    Ok((img.height(), img.width()))
}
