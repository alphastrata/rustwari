use anyhow::Result;
use log::info;
use reqwest::Client;
use std::path::Path;

use rustwari::cvutils::assemble_full_disc;
use rustwari::fileutils::{check_setup, move_completed_to_backup};
use rustwari::himawaridt::HimawariDatetime;
use rustwari::tiles::{build_tile_map, fetch_full_disc, get_x_y_from_filename, LocalTile};
use rustwari::user_config::{Config, USERCONFIG};
use rustwari::wallpaperutils::FullDisc;

async fn run(uc: &Config) -> Result<()> {
    let hwdt = HimawariDatetime::closest_to_now().await;

    let client = Client::new();
    let handles = fetch_full_disc(&client, hwdt, uc).await?;

    for h in handles
        .lock()
        .expect("Unable to lock joinhandles")
        .drain(..)
    {
        h.await?;
    }

    let tmpdir_contents = std::fs::read_dir(Path::new(&uc.tmp))?;
    let mut local_tiles = Vec::new();

    for entry in tmpdir_contents {
        let entry = entry?;
        let xy: (u32, u32) = get_x_y_from_filename(&entry, uc)?;
        local_tiles.push(LocalTile::new(xy.0, xy.1, entry.path()).await);
    }

    let tile_map = build_tile_map(local_tiles).await?;

    // Set that badboy as your wallpaper.
    let mut fulldisc: FullDisc = assemble_full_disc(tile_map, hwdt, uc).await?;
    fulldisc.resize_this(5120, 5120)?;
    let _ = fulldisc.set_this();
    info!("Sleeping for 10 minutes..."); //TODO: include the timeout as a user_config option

    std::thread::sleep(std::time::Duration::from_secs(601));
    move_completed_to_backup(&fulldisc.path)?;
    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    Config::check_config_exits(USERCONFIG)?;
    let uc = Config::new_from_yaml(USERCONFIG)?;

    check_setup(&uc).await?;

    loop {
        run(&uc).await?;
    }
    #[allow(unreachable_code)]
    Ok(())
}
