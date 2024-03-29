use crate::cvutils::{COLMAX, ROWMAX};
use crate::himawaridt::HimawariDatetime;

use anyhow::Result;
use async_recursion::async_recursion;
use bytes::Bytes;
use image::DynamicImage;
use log::error;
use reqwest::{Client, Url};
use std::sync::{Arc, Mutex};
use tokio::{sync::mpsc::Sender, task::JoinHandle};

/// Use the [`tokio`] runtime to fetch tiles in green threads.
/// Useful for getting mutiple tiles at once, use [`download_image`] for one offs.
pub async fn tokio_tile_fetcher(
    rt: RemoteTile,
    client: &Client,
    handles: &Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>,
    tx: Sender<(Bytes, RemoteTile)>,
) -> Result<(), reqwest::Error> {
    let client_c = client.clone();

    let handle = tokio::spawn(async move {
        match rt.download_image(&client_c).await {
            Ok(it) => {
                tx.send((it, rt)).await.unwrap();
            }
            Err(err) => loop {
                // Keep spamming till we get it..
                if let Ok(it) = rt.download_image(&client_c).await {
                    tx.send((it, rt)).await.unwrap();
                    break;
                }
                error!("{}", err);
            },
        }
    });

    handles.lock().unwrap().push(handle);

    Ok(())
}

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
/// Creates an Image from [`Bytes`]!
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
