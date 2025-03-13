<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center" dir="auto">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo逃走啦~"/>
</p>

<p align="center">
  <h1 align="center">OsynicDownloader 🎵</h1>
  <p align="center">Rust编写的多线程osu!谱面下载器库，下载队列基于vielpork。</p>
</p>

<p align="center">
  <a href="https://crates.io/crates/osynic_downloader" target="_blank"><img src="https://img.shields.io/crates/v/osynic_downloader"/></a>
  <a href="https://docs.rs/osynic_downloader" target="_blank"><img src="https://img.shields.io/docsrs/osynic_downloader/0.1.2"/></a>
  <a href="https://github.com/osynicite/osynic_downloader" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>

</p>

<p align="center">
  <hr />

[中文版本](README.md) | [English Version](README_EN.md)

[osynic_downloader](https://github.com/osynicite/osynic_downloader) 是一款高效的osu!谱面下载工具，基于[vielpork](https://github.com/islatri/vielpork)开发，支持两种输入格式和并行下载，专为音游玩家和多设备谱面同步打造。

![osynic_downloader.gif](https://s2.loli.net/2025/03/10/hasqOmgctyG4TWd.gif)

推荐搭配[osynic_serializer](https://github.com/osynicite/osynic_serializer)使用，实现osu!谱面的快速序列化。

![osynic_serializer.gif](https://s2.loli.net/2025/03/10/cwsgFnTEa76xiWQ.gif)

## ✨ 特性

- **双模式输入**：支持osu!谱面集ID列表和Osynic序列化生成的JSON格式
- **多下载源**：目前支持OsuDirect、OsuApiV2、SayoApi和ChimuApi共四种下载源
- **并发支持**：多线程并发下载加速（默认4线程）（请注意各osu!镜像站API的并发限制！文明使用！）
- **智能管理**：自动创建目录结构，自定义保存路径
- **可视化进度**：实时TUI进度显示（支持终端256色）
- **错误恢复**：无畏中断，自动断点续传

## 📦 安装

### 预编译版本

```bash
cargo install osynic_downloader
```

### 源码编译

```bash
git clone https://github.com/osynicite/osynic_downloader
cd osynic_downloader
cargo build --release
```

## 🚀 快速开始

### 基本使用

```bash
# 原生模式（ID列表）
osynic-dl --beatmapsets json/sets.json -o ./osu_maps -c 8

# Osynic模式（歌曲元数据）
osynic-dl --osynic-songs json/songs.json --output ./music
```

### 输入JSON示例

**sets.json**（Beatmapsets模式）:

```json
{
    "beatmapset_ids": ["114514", "1919810", "1538879"]
}
```

**songs.json**（Songs模式）（Osynic）:

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

## ⚙️ 参数详解

| 参数           | 简写 | 默认值      | 说明                                 |
| -------------- | ---- | ----------- | ------------------------------------ |
| --beatmapsets  | -b   | -           | 原生模式JSON文件路径                 |
| --osynic-songs | -n   | -           | Osynic模式JSON文件路径               |
| --source       | -s   | SayoApi     | osu!谱面下载源                       |
| --username     | -u   | -           | osu!账号（仅OsuDirect/OsuApiV2需要） |
| --password     | -p   | -           | osu!密码（仅OsuDirect/OsuApiV2需要） |
| --output       | -o   | beatmapsets | 下载目录（自动创建）                 |
| --concurrency  | -c   | 4           | 下载并发数（1-16）                   |
| --help         | -h   | -           | 显示帮助信息                         |

## 📥 支持的osu!下载源

1. **OsuDirect**：osu!官方谱面下载源（需osu账号密码，做URL传参）
2. **OsuApiV2**（暂未支持）: osu!lazer的谱面下载源（需osu账号密码，做Basic认证）
3. **SayoApi**（默认）：Sayobot谱面下载源（无需登录）
4. **ChimuApi**：Chimu.moe谱面下载源（无需登录）

## 📌 注意事项

1. 视频下载适配（no_video）暂未实现，相关选项会被忽略
2. 下载文件命名遵循`{{filename}}`命名规则
3. 使用`Ctrl+C`中断下载进程后，可重新运行恢复下载
4. 建议使用稳定的网络连接以获得最佳体验

## 🆗 作为库来使用

其实其中最有用的应该就`osynic_downloader::resolver::OsuBeatmapsetResolver`和`osynic_downloader::sources::DownloadSourceType`了，前者提供了osu!谱面集的解析器（文档参见[https://hakochest.github.io/vielpork-cn/custom-resolver/osu-beatmap-resolver.html](https://hakochest.github.io/vielpork-cn/custom-resolver/osu-beatmap-resolver.html)），后者提供了4个下载源的枚举。

首先在你的`Cargo.toml`中添加依赖，通常情况下，还是推荐和[vielpork](https://github.com/islatri/vielpork)一起使用。

```toml
[dependencies]
osynic_downloader = { version="0.1.2", default-features = false, features = ["cli"]}
vielpork = "0.1.2"
```

然后就可以为你的vielpork下载器添加osu!谱面的解析器了！

```rust
use vielpork::downloader::Downloader;
use vielpork::base::structs::DownloadOptions；
use vielpork::reporters::cli_boardcast_mpsc::CliReporterBoardcastMpsc;

use osynic_downloader::resolver::OsuBeatmapsetResolver;


// 在具体业务中
let options = DownloadOptions::default();
let reporter = CliReporterBoardcastMpsc::new(128);
let resolver = OsuBeatmapsetResolver::new();
let downloader = Downloader::new(options, Box::new(resolver), Box::new(reporter.clone()));
        
```

## 🤝 贡献指南

欢迎通过Issue提交建议或Pull Request参与开发！请确保：

- 遵循Rust官方编码规范
- 新增功能需附带测试用例
- 提交前运行`cargo fmt`和`cargo clippy`

## 📜 开源协议

本项目基于 [MIT License](LICENSE) 开源，请尊重原作者的著作权。使用osu!相关资源时请遵守[osu!社区准则](https://osu.ppy.sh/wiki/zh/Legal)。
