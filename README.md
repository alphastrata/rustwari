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

<p align="center">
<img src="https://i.imgur.com/MKQFqGY.png" alt="17:30 September 3rd 2018"/>
</p>

## Requirements:

1. rust
2. opencv
3. internet
4. _this_ repo

# Usage:

_assuming you've cloned the repo and got all the above installed properly_

1. `cd <wherever you cloned _this_ repo>`
2. `cargo build --release`
3. `./target/release/rustwari`

<p align="right">(<a href="#top">back to top</a>)</p>

## Acknowledgments

- [CY](https://github.com/Subzerofusion)
- [Scotty](https://github.com/AberrantWolf)
- [Dave](https://github.com/DTibbs)
- [NICT](https://www.nict.go.jp/index.html)

<p align="right">(<a href="#top">back to top</a>)</p>
