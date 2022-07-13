```
                         __   ^
|-| ! |\/| /\ \ /\ / /\  |_!  |
! ! | |  !/  \ \  / /  \ |\   !
                         | \    -気象衛星
rustwari

```

## About:

[himawari](https://himawari8.nict.go.jp/) is a Japanese satellite for weather monitoring, it takes an image of the full earth's disc every ten minutes, and has done so since 2015.
The highest available resolution of the tiles the disc image made up of is 550 by 550px, so when you stitch all 400 images together to make a full disc you get a whopping 11000 by 11000px 121MP image.

Rustwari, this app gets the most recent image available and sets it as your wallpaper. (after resizing it a little bit...)

## Requirements:

1. rust
2. opencv
3. internet
4. _this_ repo

# Usage:

_assuming you've cloned the repo and got all the above installed properly_

1. `cd <wherever you put rustwari>`
2. `cargo build --release`
3. `./target/release/rustwari`

## TODO:

- [] implement the user_config stuff properly to reduce number of magics/consts etc in binary
- [] can you work out if the tileset fetched is bogus, then wait a moment before trying again? (rather than go for the full 20mins behind..)
- [] on init, create the dirstructures required (/completed/ and /tmp/)
- [] Can you make the concat or resize faster? the image crate ain't exactly speedy... 5-30s for a full run, yes.. most of that is in the download
- [] black n white mode?
- [] get screen dims from user's machine and make the max resize slightly short of that
- [] is it worth even doing the resize?
- [] make it obvious that this thing's using UTC+0 so match up with how the sat keeps time
- [] random resized crop
- [] rather than resize is it possible to fetch a disc composed of smaller tiles...? n\*550^2
- [] user_config.rs is a mess, and obviously isn't working...so you need to implement that
- [] investigate maximum open files bug on maxosx 11.2.* or newer?
- [] better to have the 10min loop in the app or as a shell script... ???

## DONE:
- [x] leave it running overnight
- [x] give it to don
- [x] Break it out into files, suggest: himawaridatetime.rs, cv.rs, tiles.rs, utils.rs
- [x] what does pub(crate) do?
- [x] Fix get_x_y from filename
- [x] fetch_full_disk seems to be returning a lot of "no image" results from valid urls..
- [x] get the map builder working for full_discs.
- [x] get the map built working in the disk concatenator.
- [x] the minute initialiser in the HimaWariDateTime HWDT can fail at values under 10 >< needs fixing. when fetching 1410 the actual latest available is 1350
- [x] leave it running for 24 hours
- [x] make a run script to run it every 10 minutes.


<p align="right">(<a href="#top">back to top</a>)</p>

## Acknowledgments

- [CY](https://github.com/Subzerofusion)
- [Scotty](https://github.com/AberrantWolf)
- [Dave](https://github.com/DTibbs)
- [NICT](https://www.nict.go.jp/index.html)
<p align="right">(<a href="#top">back to top</a>)</p>
