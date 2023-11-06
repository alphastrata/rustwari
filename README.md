```
                         __   ^
|-| ! |\/| /\ \ /\ / /\  |_!  |
! ! | |  !/  \ \  / /  \ |\   !
                         | \    -気象衛星
rustwari

```

## About:

[Himawari](https://himawari8.nict.go.jp/) is a Japanese satellite for weather monitoring, it takes an image of the full earth's disc every ten minutes, and has done so since 2015.

Rustwari, this app gets the most recent of those images available and sets it as your wallpaper. (after resizing it a little bit...)

The images are a composite of `550` by `550px` square `.pngs`, of which there are `400` in total.
When you stitch all `400` images together to make a full disc you get an absolute feast for the eyes at `11000` by `11000px`, or a `121MP` image.

It's had many tweaks over the years, most recently the move to doing everything in memory (where possible), to avoid going out to disk(slow), and also moving to an `mpsc` pattern, rather than waiting fo the collections to build up before processing them. About three years ago when I originally made this it was taking about 30-35s to download, process and set an Image, whereas now it should take seconds (usually less than 10, depending on your internet).

<p align="center">
<img src="https://i.imgur.com/MKQFqGY.png" alt="17:30 September 3rd 2018"/>
</p>

## Requirements:

1. [Rust](https://www.rust-lang.org/tools/install)
2. Internet connection
3. [This repo](https://github.com/alphastrata/rustwari/)
4. A high(ish) ulmit `ulimit -n 10240` ought to do it. (the app will try to set it for you.)
5. _If you're a hyprpaper, if you're the hyprland type..._

# Usage:

_assuming you've cloned the repo and got all the above installed properly_
_the app assumes you've two dirs (somewhere) called `completed` and `backup`, ideally they're pathed in your config.yml but, if you don't have one or the dirs the app will create them for you_.

```
cd rustwari
cargo build --release # hyprland+hyprpaper users should use the --features "hypr" build flag
./target/release/rustwari --help

```

> NOTE: just running `./target/release/rustwari` will run the app with default args.

which will give you this menu:

```bash
Rustwari, this app gets the most recent image available from the Himawari8 dataset, and, sets it as your wallpaper. (after resizing it a little bit...).

Himawari8 is a Japanese satellite for weather monitoring, it takes an image of the full earth's disc every ten minutes, and has done so since 2015.
The images are distributed are 550 by 550px .pngs of which there are a whopping 400.

When you stitch all 400 images together to make a full disc you get an absolute feast for the eyes at 11000 by 11000px, or a 121MP image.


Usage: rustwari [OPTIONS]

Options:
  -q, --quiet                          Enjoy a silent app with nothing more than a progressbar
      --hypr                           Enjoy a silent app with nothing more than a progressbar
  -v, --verbose                        Not reccomended unless developing
      --completed-dir <COMPLETED_DIR>  <WIP>Override the `completed` dir in your existing config file
  -c, --config-file <CONFIG_FILE>      <WIP>If you have a specific path to the config.yml this app uses to provide specific paths to where you want temporary, and, completed Images stored
  -r, --resize                         Resize the image after processing, default is false
      --open                           Open the image after completing it's retrival
      --oneshot <ONESHOT>              Get the entry for YYYY-MM-DD HH:MM, note you'll need to wrap it all in ' or "s
      --backup                         Do you want the app to backup used backgrounds to an alternative directory?
  -h, --help                           Print help
  -V, --version                        Print version
```

So, for a oneshot (which automatically wil try to open the downloaded image)

```bash
rustwari --oneshot "2018-08-18 11:00"
```

If you're running in a cron job or something:
`rustwari -q`
if you're getting errors try:
`rustwari -v` #you can also check the `.log` file it makes.

> Anything marked with <WIP> is a work in progress/Yet to be implemented.

_\* if you're getting a lot of crashes due to 'too many open files...' try `ulimit -n SOMEHIGHNUMBER`_

<p align="right">(<a href="#top">back to top</a>)</p>

## Acknowledgments

- [CY](https://github.com/Subzerofusion)
- [Scotty](https://github.com/AberrantWolf)
- [Dave](https://github.com/DTibbs)
- [NICT](https://www.nict.go.jp/index.html)

<p align="right">(<a href="#top">back to top</a>)</p>
