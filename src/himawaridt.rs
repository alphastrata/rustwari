use anyhow::Result;
use chrono::{Datelike, Timelike, Utc};
use std::path::Path;
use url::{ParseError, Url};

const URLBASE: &str = r#"https://himawari8.nict.go.jp/img/D531106/20d/550/"#;

/// The structure holding all the temporal info about a tile
/// used to generate filenames and urls
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct HimawariDatetime {
    pub year: u32, // 2015 to now *NOTE:2015 July 7th 9:50 is the first day available from the dataset
    pub month: u32, // 1-12
    pub day: u32,  // 1-31
    pub h: u32,    // 0 to 23
    pub m: u32, // can only increment from 0 to 50, by 10 at time *NOTE:1640 never works so strip it?
}
impl HimawariDatetime {
    /// Constructs a HimawariDatetime
    /// # Arguments
    /// * `year` - The year
    /// * `month` - The month
    /// * `day` - The day
    /// * `h` - The hour
    /// * `m` - The minute
    pub(crate) async fn _new(year: u32, month: u32, day: u32, h: u32, m: u32) -> Self {
        let now = Utc::now();
        if year <= 2015 && month < 8 // date is outside the dataset
            || year > now.year() as u32  // date is in the future
            || month > 12
            || day > 31
            || h > 24
            || m > 60
        {
            println!("Invalid datetime, returning default, which is the first day of operation available in the dataset.");
            println!("YOU ENTERED:{}/{}/{} {}:{}", year, month, day, h, m);
            return Self {
                year: 2015,
                month: 7,
                day: 7,
                h: 1,
                m: 50,
            };
        };
        let m: u32 = now.minute() as u32 - (now.minute() as u32 % 10);
        Self {
            year,
            month,
            day,
            h,
            m,
        }
    }
    /// Helper to build pretty filenames for complete disks
    pub(crate) fn pretty_filename(&self) -> String {
        format!(
            "fulldisc-{}-{:02}-{:02} {:02}_{:02}.png",
            self.year, self.month, self.day, self.h, self.m
        )
    }
    /// Helper to get a HWDT back out of an existing file, usually used on a fulldisc
    pub(crate) fn _from_path(path: &Path) -> Self {
        let p = path.to_str().expect("unable to parse PathBuf");

        // example filename : fulldisc-2022-2-3 0_30.png
        let p_split = p.split('-').collect::<Vec<&str>>();
        let year = p_split[1].parse::<u32>().expect("unable to parse year");
        let month = p_split[2].parse::<u32>().expect("unable to parse month");
        let day = p_split[3].parse::<u32>().expect("unable to parse day");
        let h_m = p_split[4].split('_').collect::<Vec<&str>>();
        let h = h_m[0].parse::<u32>().expect("unable to parse hour");
        let m = h_m[1].parse::<u32>().expect("unable to parse minute");

        Self {
            year,
            month,
            day,
            h,
            m,
        }
    }

    /// Constructs a HimawariDatetime closest to your current time, minus 20 minutes.
    pub(crate) async fn closest_to_now() -> Self {
        let now = Utc::now() - chrono::Duration::minutes(20);
        Self {
            year: now.year() as u32,
            month: now.month(),
            day: now.day(),
            h: now.hour() as u32,
            m: now.minute() as u32 - (now.minute() as u32 % 10),
        }
    }
    /// builds valid urls for tiles from HimawariDatetime
    /// # Arguments:
    /// * `hmtd` - a valid HimawariDatetime
    /// * `x` - x coordinate of tile
    /// * `y` - y coordinate of tile
    pub(crate) async fn get_url(&self, x: u32, y: u32) -> Result<Url, ParseError> {
        let url = Url::parse(
            &format!(
                "{}{}/{:02}/{:02}/{:02}{:02}00_{}_{}.png",
                URLBASE, self.year, self.month, self.day, self.h, self.m, x, y
            )[..],
        )?;
        Ok(url)
    }
}
