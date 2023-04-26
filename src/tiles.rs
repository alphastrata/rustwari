use crate::cvutils::{COLMAX, ROWMAX};
use crate::himawaridt::HimawariDatetime;

use anyhow::Result;
use async_recursion::async_recursion;
use bytes::Bytes;
use image::DynamicImage;
use log::{debug, error};
use reqwest::{Client, Url};
use std::sync::{Arc, Mutex};
use tokio::{sync::mpsc::Sender, task::JoinHandle};
use url::ParseError;

/// Use the [`tokio`] runtime to fetch tiles in green threads.
/// Useful for getting mutiple tiles at once, use [`download_image`] for one offs.
pub async fn tokio_tile_fetcher(
    rt: RemoteTile,
    client: &Client,
    handles: &Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>,
    tx: Sender<(Bytes, RemoteTile)>,
) -> Result<(), reqwest::Error> {
    let client = client.clone();

    let handle = tokio::spawn(async move {
        match tx
            .send((
                rt.download_image(&client).await.unwrap_or_else(|e| {
                    debug!(
                        "Unexpected failure in downloading {:?}\n{e:#?}",
                        rt.url.as_str()
                    );
                    panic!(
                        "Unexpected failure in downloading {:?}\n{e:#?}",
                        rt.url.as_str()
                    );
                }),
                rt,
            ))
            .await
        {
            Ok(_) => {}
            Err(e) => error!("{}", e),
        }
    });

    handles.lock().unwrap().push(handle);

    Ok(())
}

/// A helper to fetch an entire disc's worth of tiles.
/// Note: This fetch is self-recursive, so it will check that all 400 tiles are present, and, if not rerun the fetch.
// #[async_recursion]
// pub async fn fetch_full_disc(
//     client: &Client,
//     hwdt: HimawariDatetime,
//     tx: Sender<(Bytes, RemoteTile)>,
// ) -> Result<Arc<Mutex<Vec<JoinHandle<()>>>>> {
//     let handles = Arc::new(Mutex::new(Vec::new()));

//     for x in 0..ROWMAX {
//         for y in 0..COLMAX {
//             let url = hwdt.get_url(x, y).await?;
//             let rt = RemoteTile::new(x, y, url).await;
//             tokio_tile_fetcher(rt, client, &handles, tx.clone()).await?;
//         }
//     }

//     Ok(handles)
// }

#[async_recursion]
pub async fn fetch_full_disc(
    client: &Client,
    hwdt: HimawariDatetime,
    tx: Sender<(Bytes, RemoteTile)>,
) -> Result<Arc<Mutex<Vec<JoinHandle<()>>>>> {
    use futures::future::join_all;
    let handles = Arc::new(Mutex::new(Vec::new()));

    let fetch_tasks = (0..ROWMAX)
        .flat_map(|x| (0..COLMAX).map(move |y| (x, y)))
        .map(|(x, y)| {
            let client = client.clone();
            let tx = tx.clone();
            let handles = handles.clone();
            async move {
                let url = hwdt.get_url(x, y).unwrap();
                let rt = RemoteTile::new(x, y, url).await;
                tokio_tile_fetcher(rt, &client, &handles, tx).await
            }
        })
        .collect::<Vec<_>>();

    join_all(fetch_tasks).await;

    Ok(handles)
}
#[deprecated] // we no longer keep any intermediary files on disk.
/// Identical to the [`RemoteTile`] except that this one exists on disk.
#[derive(Debug, Clone)]
pub struct LocalTile {
    pub x: u32,
    pub y: u32,
    pub bytes: Option<DynamicImage>,
}

#[deprecated]
impl LocalTile {
    /// A getter for the x, and y values representing the tile's location.
    pub fn xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}
/// Creates an Image from Bytes!
pub(crate) fn img_from(b: Bytes) -> DynamicImage {
    image::load_from_memory(&b).unwrap() //TODO: fix this, error handling.
}

/// Hold the data for a single tile, prior to fetching it from the dataset
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RemoteTile {
    pub x: u32,
    pub y: u32,
    pub url: Url,
}
impl RemoteTile {
    pub async fn new(x: u32, y: u32, url: Url) -> Self {
        if x <= 19 && y <= 19 {
            Self { x, y, url }
        } else {
            Self {
                x: 0,
                y: 0,
                // This is not a safe unwrap.
                url: Url::parse("").unwrap(),
            }
        }
    }
    /// Downloads the tile to the specified path
    pub async fn download_image(&self, client: &Client) -> Result<Bytes, reqwest::Error> {
        let url = self.url.clone();
        let resp = client.get(url).send().await?.error_for_status()?;

        let bytes = resp.bytes().await.unwrap();

        Ok(bytes)
    }
}
