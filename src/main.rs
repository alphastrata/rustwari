#![allow(unreachable_patterns)]

use anyhow::Result;
use chrono::NaiveDateTime;
use log::debug;
use open::that;
use tokio::sync::mpsc;

use reqwest::Client;
use rustwari::cli::Cli;
use rustwari::cvutils::assemble_full_disc;
use rustwari::fileutils::{check_setup, move_completed_to_backup};
use rustwari::himawaridt::HimawariDatetime;
use rustwari::termite::setup_logger;
use rustwari::tiles::fetch_full_disc;
use rustwari::user_config::{Config, USERCONFIG};
use rustwari::wallpaperutils::FullDisc;

async fn run(client: &Client, uc: &Config, cli: &Cli) -> Result<()> {
    let t1 = std::time::Instant::now();
    let hwdt = HimawariDatetime::closest_to_now();
    let (tx, rx) = mpsc::channel(400);

    let handles = fetch_full_disc(client, hwdt, tx).await?;

    #[allow(clippy::await_holding_lock)] // We are using a mutex clippy!
    for h in handles
        .lock()
        .expect("Unable to lock joinhandles")
        .drain(..)
    {
        #[allow(clippy::await_holding_lock)]
        h.await?;
    }

    // Set that badboy as your wallpaper.
    let mut fulldisc: FullDisc = assemble_full_disc(hwdt, uc, cli, rx).await?;

    if cli.resize {
        fulldisc.resize_this(5120, 5120)?; //TODO: const
    }

    fulldisc.set_this()?;

    move_completed_to_backup(fulldisc.path, uc)?;

    if cli.verbose {
        debug!("{}", t1.elapsed().as_secs_f64());
    }

    std::thread::sleep(std::time::Duration::from_secs(601));
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let cli = Cli::init();

    //Setup
    Config::check_config_exits(USERCONFIG).expect("User Config doesn't exist :(");
    let uc = Config::new_from_yaml(USERCONFIG)?;

    if cli.verbose {
        setup_logger(true).expect("FAILED TO INIT LOGGER!");
        debug!("{uc:#?}");
    }

    check_setup(&uc).await.expect("Setup is borked...");

    if cli.oneshot.is_some() {
        _ = run_oneshot(&client, &cli, &uc).await;
    } else {
        // Run indefnitely.
        loop {
            run(&client, &uc, &cli).await?;
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}

async fn run_oneshot(client: &Client, cli: &Cli, uc: &Config) -> Result<()> {
    if let Some(oneshot_str) = cli.oneshot.clone() {
        let hwdt = NaiveDateTime::parse_from_str(&oneshot_str, "%Y-%m-%d %H:%M")
            .unwrap_or_else(|e| panic!("{e:#?}\n{oneshot_str}"));

        let (tx, rx) = mpsc::channel(400);

        let handles = fetch_full_disc(client, hwdt.into(), tx).await?;

        for h in handles
            .lock()
            .expect("Unable to lock joinhandles")
            .drain(..)
        {
            h.await?;
        }

        // Set that badboy as your wallpaper.
        let mut fulldisc: FullDisc = assemble_full_disc(hwdt.into(), uc, cli, rx).await?;

        if cli.resize {
            fulldisc.resize_this(5120, 5120)?;
        }

        that(fulldisc.path)?;
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustwari::tiles::RemoteTile;

    use bytes::Bytes;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    fn setup() -> (
        Cli,
        Client,
        Config,
        (
            mpsc::Sender<(Bytes, RemoteTile)>,
            mpsc::Receiver<(Bytes, RemoteTile)>,
        ),
    ) {
        let cli = Cli {
            quiet: false,
            verbose: true,
            completed_dir: None,
            config_file: None,
            resize: true,
            open: false,
            oneshot: None,
        };

        let client = Client::new();

        Config::check_config_exits(USERCONFIG).expect("User Config doesn't exist :(");
        let uc = Config::new_from_yaml(USERCONFIG).unwrap();

        let (tx, rx) = mpsc::channel(400);
        (cli, client, uc, (tx, rx))
    }

    #[tokio::test]
    async fn run_now() {
        let (cli, client, uc, (tx, rx)) = setup();

        let t1 = std::time::Instant::now();
        let hwdt = HimawariDatetime::new(2018, 8, 18, 9, 0).await;

        let handles = fetch_full_disc(&client, hwdt, tx).await.unwrap();

        for h in handles
            .lock()
            .expect("Unable to lock joinhandles")
            .drain(..)
        {
            h.await.unwrap();
        }

        // Set that badboy as your wallpaper.
        let mut fulldisc: FullDisc = assemble_full_disc(hwdt, &uc, &cli, rx).await.unwrap();

        if cli.resize {
            fulldisc.resize_this(5120, 5120).unwrap();
        }

        fulldisc.set_this().unwrap();

        eprintln!("Elapsed: {}s", t1.elapsed().as_secs_f64());
    }

    #[tokio::test]
    async fn oneshot_mode() {
        let (cli, client, uc, (tx, rx)) = setup();

        let oneshot_str = "2022-09-21 00:10";
        let hwdt = NaiveDateTime::parse_from_str(&oneshot_str, "%Y-%m-%d %H:%M")
            .unwrap_or_else(|e| panic!("{e:#?}\n{oneshot_str}"));

        let handles = fetch_full_disc(&client, hwdt.into(), tx).await.unwrap();

        for h in handles
            .lock()
            .expect("Unable to lock joinhandles")
            .drain(..)
        {
            h.await.unwrap();
        }

        // Set that badboy as your wallpaper.
        let mut fulldisc: FullDisc = assemble_full_disc(hwdt.into(), &uc, &cli, rx)
            .await
            .unwrap();

        if cli.resize {
            fulldisc.resize_this(5120, 5120).unwrap();
        }

        let mut fd = File::open(fulldisc.path).unwrap();

        let mut result = vec![];
        fd.read_to_end(&mut result).unwrap();

        let expected = Path::new("./test_data/fulldisc-2022-09-21 00_10.png");
        let mut test_fd = File::open(expected).unwrap();

        let mut expected = vec![];
        test_fd.read_to_end(&mut expected).unwrap();

        assert_eq!(expected, result);
    }
}
