<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo逃走啦~"/>
</p>

<p align="center">
  <h1 align="center">OsynicDownloader 🎵</h1>
  <p align="center">Osu! beatmap downloader library written in Rust with multi-threading support, download queue based on vielpork.</p>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue"/></a>
  <a href="https://crates.io/crates/osynic_downloader" target="_blank"><img src="https://img.shields.io/crates/v/osynic_downloader"/></a>
  <a href="https://docs.rs/osynic_downloader" target="_blank"><img src="https://img.shields.io/docsrs/osynic_downloader/0.1.2"/></a>
  <a href="https://github.com/osynicite/osynic_downloader" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>

</p>

<p align="center">
  <hr />

[中文版本](README.md) | [English Version](README_EN.md)

[osynic_downloader](https://github.com/osynicite/osynic_downloader) is an efficient osu! beatmap downloader tool based on [vielpork](https://github.com/islatri/vielpork), supporting two input formats and parallel downloading, designed for rhythm game players and beatmap managers.

![osynic_downloader.gif](https://s2.loli.net/2025/03/10/hasqOmgctyG4TWd.gif)

Recommended to use with [osynic_serializer](https://github.com/osynicite/osynic_serializer) to achieve fast serialization of osu! beatmaps.

![osynic_serializer.gif](https://s2.loli.net/2025/03/10/cwsgFnTEa76xiWQ.gif)

## ✨ Features

- **Dual-mode input**: Supports native osu! beatmap set ID list and custom Osynic serialization generated format
- **Multiple download sources**: Currently supports four download sources: OsuDirect, OsuApiV2, SayoApi, and ChimuApi
- **Concurrency support**: Multi-threaded concurrent downloading acceleration (default 4 threads) (please note the concurrency limit of various osu! mirror site APIs! Use it properly!)
- **Intelligent management**: Automatically create directory structure, custom save path
- **Visual progress**: Real-time TUI progress display (supports terminal 256 colors)
- **Error recovery**: State recovery mechanism ensures download integrity

## 📦 Installation

### Precompiled version

```bash
cargo install osynic_downloader
```

### Source code compilation

```bash
git clone https://github.com/osynicite/osynic_downloader
cd osynic_downloader
cargo build --release
```

## 🚀 Quick Start

### Basic usage

```bash
# Native mode (ID list)
osynic-dl --beatmapsets json/sets.json -o ./osu_maps -c 8
# Osynic mode (song metadata)
osynic-dl --osynic-songs json/songs.json --output ./music
```

### Configuration file example

**sets.json** (native mode):

```json
{
    "beatmapset_ids": ["114514", "1919810", "1538879"]
}
```

**songs.json** (Osynic mode):

```json
[
  {
    "song_id": 1985060,
    "artist_name": "ヒトリエ",
    "mapper_name": "flake",
    "song_name": "日常と地球の額縁 (wowaka x 初音ミク Edit)",
    "no_video": false
  },
    {
    "song_id": 1997071,
    "artist_name": "ナブナ",
    "mapper_name": "Ryuusei Aika",
    "song_name": "始発とカフカ",
    "no_video": false
  }
]
```

## 📜 Command Line Options

| Option         | Short | Default     | Description                    |
| -------------- | ----- | ----------- | ------------------------------ |
| --beatmapsets  | -b    | -           | Path to native mode JSON file  |
| --osynic-songs | -n    | -           | Path to Osynic mode JSON file  |
| --source       | -s    | SayoApi     | osu! beatmap download source   |
| --username     | -u    | -           | osu! account (only for OsuDirect/OsuApiV2) |
| --password     | -p    | -           | osu! password (only for OsuDirect/OsuApiV2) |
| --output       | -o    | beatmapsets | Download directory (auto-created) |
| --concurrency  | -c    | 4           | Download concurrency (1-16)    |
| --help         | -h    | -           | Display help information        |

## 📥 Supported osu! Download Sources

1. **OsuDirect**: Official osu! beatmap download source (osu username and password required, URL parameters)
2. **OsuApiV2**(Not available yet): osu!lazer beatmap download source (osu username and password required, Basic authentication)
3. **SayoApi** (default): Sayobot beatmap download source (no login required)
4. **ChimuApi**: Chimu.moe beatmap download source (no login required)

## 📌 Notes

1. Video download adaptation (no_video) is not yet implemented, and related options will be ignored
2. Download file naming follows the `{{filename}}` naming rule
3. Interrupting the download process with `Ctrl+C` and then rerunning will resume the download
4. It is recommended to use a stable network connection for the best experience

## 🆗 Use as a lib

The most useful part of it should be `osynic_downloader::resolver::OsuBeatmapsetResolver` and `osynic_downloader::sources::DownloadSourceType`, the former provides a resolver for osu! beatmap sets (see [https://hakochest.github.io/vielpork-en/custom-resolver/osu-beatmap-resolver.html](https://hakochest.github.io/vielpork-en/custom-resolver/osu-beatmap-resolver.html) for documentation), and the latter provides an enumeration of four download sources.

First, add dependencies to your `Cargo.toml`. In most cases, it is recommended to use it with [vielpork](https://github.com/islatri/vielpork).

```toml
[dependencies]
osynic_downloader = { version="0.1.2", default-features = false, features = ["cli"]}
vielpork = "0.1.2"
```

Then you can add an osu! beatmap resolver to your vielpork downloader!

```rust
use vielpork::downloader::Downloader;
use vielpork::base::structs::DownloadOptions；
use vielpork::reporters::cli_boardcast_mpsc::CliReporterBoardcastMpsc;

use osynic_downloader::resolver::OsuBeatmapsetResolver;


// In your business logic
let options = DownloadOptions::default();
let reporter = CliReporterBoardcastMpsc::new(128);
let resolver = OsuBeatmapsetResolver::new();
let downloader = Downloader::new(options, Box::new(resolver), Box::new(reporter.clone()));
        
```

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

- Follow the official Rust coding style
- Add test cases for new features
- Run `cargo fmt` and `cargo clippy` before submitting

## 📜 License

This project is open-sourced under the [MIT License](LICENSE). Please respect the original author's copyright. When using osu! related resources, please follow the [osu! community guidelines](https://osu.ppy.sh/wiki/zh/Legal).
