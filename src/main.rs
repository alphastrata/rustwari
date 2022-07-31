use anyhow::Result;
use reqwest::Client;
use std::path::Path;

mod cvutils;
mod fileutils;
mod himawaridt;
mod tiles;
mod user_config;
mod wallpaperutils;

use cvutils::assemble_full_disc;
use fileutils::{check_setup, move_completed_to_backup};
use himawaridt::HimawariDatetime;
use tiles::{build_tile_map, fetch_full_disc, get_x_y_from_filename, LocalTile};
use user_config::{Config, USERCONFIG};
use wallpaperutils::FullDisc;

async fn run() -> Result<()> {
    let user_config = Config::new_from_yaml(USERCONFIG)?;

    let hwdt = HimawariDatetime::closest_to_now().await;

    let client = Client::new();
    let handles = fetch_full_disc(&client, hwdt, &user_config.tmp).await?;

    for h in handles
        .lock()
        .expect("Unable to lock joinhandles")
        .drain(..)
    {
        h.await?;
    }

    let tmpdir_contents = std::fs::read_dir(Path::new(&user_config.tmp))?;
    let mut local_tiles = Vec::new();
    for entry in tmpdir_contents {
        let entry = entry.unwrap();
        let path = entry.path();
        let xy: (u32, u32) = get_x_y_from_filename(path.clone())?;
        local_tiles.push(LocalTile::new(xy.0, xy.1, path.to_path_buf()).await);
    }

    let tile_map = build_tile_map(local_tiles).await?;

    // Set that badboy as your wallpaper.
    let mut fulldisc: FullDisc = assemble_full_disc(tile_map, hwdt).await?;
    fulldisc.resize_this(5120, 5120)?;
    let _ = fulldisc.set_this();
    println!("Sleeping for 10 minutes...");
    std::thread::sleep(std::time::Duration::from_secs(601));
    move_completed_to_backup(&fulldisc.path)?;
    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    check_setup().await?;
    loop {
        run().await?;
    }
    #[allow(unreachable_code)]
    Ok(())
}
