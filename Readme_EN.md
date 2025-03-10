# Usage Example: OsynicDownloader

[osynic_downloader](https://crates.io/crates/osynic_downloader) is an efficient osu! beatmap downloader tool based on [vielpork](https://crates.io/crates/vielpork), supporting two input formats and parallel downloading, designed for rhythm game players and beatmap managers.

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

## Supported osu! Download Sources

1. **OsuDirect**: Official osu! beatmap download source (osu username and password required)
2. **OsuApiV2**: osu!lazer beatmap download source (osu username and password required, Basic authentication)
3. **SayoApi** (default): Sayobot beatmap download source (no login required)
4. **ChimuApi**: Chimu.moe beatmap download source (no login required)

## 📌 Notes

1. Video download adaptation (no_video) is not yet implemented, and related options will be ignored
2. Download file naming follows the `{{filename}}` naming rule
3. Interrupting the download process with `Ctrl+C` and then rerunning will resume the download
4. It is recommended to use a stable network connection for the best experience

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

- Follow the official Rust coding style
- Add test cases for new features
- Run `cargo fmt` and `cargo clippy` before submitting

## 📜 License

This project is open-sourced under the [MIT License](LICENSE). Please respect the original author's copyright. When using osu! related resources, please follow the [osu! community guidelines](https://osu.ppy.sh/wiki/zh/Legal).
