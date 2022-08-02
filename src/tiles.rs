use anyhow::Error;
use anyhow::Result;
use async_recursion::async_recursion;
use log::info;
use reqwest::Client;
use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinHandle;
use url::Url;

use crate::{
    fileutils::{exists, remove_failed_and_rerun},
    himawaridt::HimawariDatetime,
    user_config::Config,
};

const URLBASE: &str = "https://himawari8.nict.go.jp/img/D531106/20d/550/"; //2018/08/18/161000_17_3.png";
const ROWMAX: u32 = 20;
const COLMAX: u32 = 20;

/// Use the tokio runtime to fetch tiles in green threads.
/// Useful for getting mutiple tiles at once, use download_image() for one offs.
/// #Arguments:
/// * `rt` RemoteTile
/// * `hmtd` - a valid HimawariDatetime
/// * `client` - Client
/// * `handles` a Vec<JoinHandle<Result<()>>>
pub(crate) async fn tokio_tile_fetcher(
    rt: RemoteTile,
    hwdt: HimawariDatetime,
    client: &Client,
    handles: &Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>,
    tmp: &str,
) {
    let client = client.clone();
    let tmp = tmp.to_owned();
    let handle = tokio::spawn(async move {
        rt.download_image(hwdt, &client, &tmp)
            .await
            .expect(&format!("Failure on:{:?}", hwdt)); // figure it's ok to destroy the hwdt here?
    });

    handles.lock().unwrap().push(handle);
}

/// Helper to return the x and y values from a given path
pub(crate) fn get_x_y_from_filename(p: &DirEntry, uc: &Config) -> Result<(u32, u32), Error> {

    let p = p.path();
    let mut pbstr = p.to_str().expect("unable to get a str from provided path");

    pbstr = pbstr
        .strip_prefix(&uc.tmp)
        .expect("unable to strip /tmp/ from Path");

    let x = pbstr.split(" R").collect::<Vec<_>>()[1]
        .to_string()
        .split('_')
        .collect::<Vec<_>>()[0]
        .parse::<u32>()?;

    let y = pbstr.split('C').collect::<Vec<_>>()[1]
        .to_string()
        .replace(".png", "")
        .parse::<u32>()?;

    Ok((x, y))
}
/// Builds a hashmap of LocalTiles, where the key is the x,y coordinate of the tile.
/// The value is the file loaded into memory with imgcodecs::imread() from the opencv library.
pub(crate) async fn build_tile_map(
    tiles: Vec<LocalTile>,
) -> Result<HashMap<(u32, u32), LocalTile>> {
    let mut m: HashMap<(u32, u32), LocalTile> = HashMap::new();

    for tile in tiles.into_iter() {
        if tile.get_size_on_disk().await > 0 {
            //NOTE: actual size of a failed tile is around 200bytes
            m.insert((tile.x, tile.y), tile);
        } else {
            info!("Tile is not ok");
            info!("{:#?}", tile);
        }
    }
    Ok(m)
}

/// A helper to fetch an entire disc's worth of tiles.
/// Note: This fetch is self-recursive, so it will check that all 400 tiles are present, and, if not rerun the fetch.
/// #Arguments:
/// * `client` - Client
/// * `hmtd` - a valid HimawariDatetime
#[async_recursion]
pub(crate) async fn fetch_full_disc(
    client: &Client,
    hwdt: HimawariDatetime,
    uc: &Config,
) -> Result<Arc<Mutex<Vec<JoinHandle<()>>>>> {
    let handles = Arc::new(Mutex::new(Vec::new()));

    for x in 0..ROWMAX {
        for y in 0..COLMAX {
            let url = hwdt.get_url(x, y).await?;
            let rt = RemoteTile::new(x, y, url).await;
            tokio_tile_fetcher(rt, hwdt, client, &handles, &uc.tmp).await;
        }
    }
    if remove_failed_and_rerun(uc)? > 0 {
        std::thread::sleep(Duration::from_millis(250));
        fetch_full_disc(client, hwdt, uc).await?;
    }
    Ok(handles)
}
/// Identical to the RemoteTile except that this one exists on disk.
#[derive(Debug, Clone)]
pub(crate) struct LocalTile {
    pub x: u32,
    pub y: u32,
    pub path: PathBuf,
}

impl LocalTile {
    pub(crate) async fn new(x: u32, y: u32, path: PathBuf) -> Self {
        if exists(path.clone()).await {
            Self { x, y, path }
        } else {
            // TODO: Go and get the tile they're asking for.
            panic!("{} Tile does not exist", path.to_str().unwrap());
        }
    }
    pub(crate) fn path_as_str(&self) -> &str {
        self.path.to_str().unwrap() //FIXME: fix this.
    }
    /// A failed tile will be 0 bytes, a disc of failures stitched will be <3mb.
    pub(crate) async fn get_size_on_disk(&self) -> usize {
        let p = self.path_as_str();
        let metadata = tokio::fs::metadata(p)
            .await
            .expect(&format!("Unable to retrieve metadata from {:?}", self.path.to_str())[..]);
        let size = metadata.len() as usize;
        if size < 200 {
            info!("WARNING:{} is {} bytes.", p, size);
        }
        size
    }
    /// A getter for the x, and y values representing the tile's location.
    pub(crate) async fn get_xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}
/// A struct to hold the data for a single tile, prior to fetching it from the dataset
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct RemoteTile {
    pub x: u32,
    pub y: u32,
    pub url: Url,
}
impl RemoteTile {
    /// Constructs a new RemoteTile
    /// #Arguments:
    /// * `x` - x coordinate of the tile
    /// * `y` - y coordinate of the tile
    pub(crate) async fn new(x: u32, y: u32, url: Url) -> Self {
        if x <= 19 && y <= 19 {
            Self { x, y, url }
        } else {
            Self {
                x: 0,
                y: 0,
                url: Url::parse("").unwrap(),
            }
        }
    }
    /// Downloads the tile to the specified path
    /// #Arguments:
    /// * `hmtd` - a HimawariDatetime struct
    /// * `client` - Client
    /// it produces files that look like this: `2018-08-18 161000_17_3.png`
    pub(crate) async fn download_image(
        &self,
        hwdt: HimawariDatetime,
        client: &Client,
        tmp: &String,
    ) -> Result<(), anyhow::Error> {
        let filename = format!(
            "{}/{}-{}-{} {:02}{:02} R{}_C{}.png",
            &tmp, hwdt.year, hwdt.month, hwdt.day, hwdt.h, hwdt.m, self.x, self.y
        );

        let url = self.url.clone();
        info!("Attempting to download {}", url);
        if !exists(Path::new(&filename).to_path_buf()).await {
            let resp = client.get(url).send().await?.error_for_status()?;
            let mut file = File::create(&filename).await?;
            file.write_all(&resp.bytes().await?).await?;
        }

        Ok(())
    }
}
