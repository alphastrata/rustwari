use anyhow::Error;
use anyhow::Result;
use chrono::NaiveDateTime;
use chrono::{Datelike, Timelike, Utc};
use log::debug;
use log::{error, warn};
use std::path::Path;
use url::{Url};

const URLBASE: &str = r#"https://himawari8.nict.go.jp/img/D531106/20d/550/"#;

/// The structure holding all the temporal info about a tile
/// used to generate filenames and [`url`]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HimawariDatetime {
    //TODO: these would all fit in a u8...
    pub year: u32, // 2015 to now *NOTE:2015 July 7th 9:50 is the first day available from the dataset
    pub month: u32, // 1-12
    pub day: u32,  // 1-31
    pub h: u32,    // 0 to 23
    pub m: u32, // can only increment from 0 to 50, by 10 at time *NOTE:1640 never works so strip it?
}
impl HimawariDatetime {
    /// Constructs a [`HimawariDatetime`]
    pub async fn new(year: u32, month: u32, day: u32, h: u32, m: u32) -> Self {
        let now = Utc::now();
        if year <= 2015 && month < 8 // date is outside the dataset
            || year > now.year() as u32  // date is in the future
            || month > 12
            || day > 31
            || h > 24
            || m > 60
        {
            error!("Invalid datetime, returning default, which is the first day of operation available in the dataset.");
            error!("YOU ENTERED:{}/{}/{} {}:{}", year, month, day, h, m);
            warn!("The earliest possible HimawariDatetime will be created for you.");
            return Self {
                year: 2015,
                month: 7,
                day: 7,
                h: 1,
                m: 50,
            };
        };

        // clamp the minutes
        let m: u32 = now.minute() - (now.minute() % 20);

        Self {
            year,
            month,
            day,
            h,
            m,
        }
    }
    /// Helper to build pretty filenames for complete disks
    pub fn pretty_filename(&self) -> String {
        format!(
            "fulldisc-{}-{:02}-{:02} {:02}_{:02}.png",
            self.year, self.month, self.day, self.h, self.m
        )
    }
    /// Helper to get a HWDT back out of an existing file, usually used on a fulldisc
    pub fn from_path(path: &Path) -> Result<Self, Error> {
        let p = path.to_str().expect("unable to parse PathBuf");

        // example filename : fulldisc-2022-2-3 0_30.png
        let p_split = p.split('-').collect::<Vec<&str>>();
        let year = p_split[1].parse::<u32>()?;
        let month = p_split[2].parse::<u32>()?;
        let day = p_split[3].parse::<u32>()?;
        let h_m = p_split[4].split('_').collect::<Vec<&str>>();
        let h = h_m[0].parse::<u32>()?;
        let m = h_m[1].parse::<u32>()?;

        Ok(Self {
            year,
            month,
            day,
            h,
            m,
        })
    }

    /// Constructs a [`HimawariDatetime`] closest to your current time, minus 20 minutes.
    pub fn closest_to_now() -> Self {
        let now = Utc::now() - chrono::Duration::minutes(20);
        debug!("closest_to_now() set at: {}", &now);
        Self {
            year: now.year() as u32,
            month: now.month(),
            day: now.day(),
            h: now.hour(),
            m: now.minute() - (now.minute() % 10),
        }
    }
    /// builds valid [`url`]
    pub fn get_url(&self, x: u32, y: u32) -> Result<Url> {
        let url = Url::parse(&format!(
            "{}{}/{:02}/{:02}/{:02}{:02}00_{}_{}.png",
            URLBASE, self.year, self.month, self.day, self.h, self.m, x, y
        ))?;

        Ok(url)
    }
}
impl From<NaiveDateTime> for HimawariDatetime {
    fn from(dt: NaiveDateTime) -> Self {
        Self {
            year: dt.year() as u32,
            month: dt.month(),
            day: dt.day(),
            h: dt.hour(),
            m: dt.minute(),
        }
    }
}
