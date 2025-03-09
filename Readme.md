# OsynicDownloader 🎵

[![Rust](https://img.shields.io/badge/Rust-1.65%2B-blue)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

一款高效的osu!谱面下载工具，支持两种输入格式和并行下载，专为音游玩家和谱面管理者打造。

## ✨ 特性

- **双模式输入**：支持原生osu!谱面集ID列表和Osynic序列化生成的定制格式
- **并发支持**：多线程并发下载加速（默认4线程）（请注意各osu!镜像站API的并发限制！文明使用！）
- **智能管理**：自动创建目录结构，自定义保存路径
- **可视化进度**：实时TUI进度显示（支持终端256色）
- **错误恢复**：状态恢复机制保障下载完整性

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

### 配置文件示例

**maps.json**（原生模式）:

```json
{
    "beatmapset_ids": ["114514", "1919810", "1538879"]
}
```

**songs.json**（Osynic模式）:

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

| 参数           | 简写 | 默认值      | 说明                           |
| -------------- | ---- | ----------- | ------------------------------ |
| --beatmapsets  | -b   | -           | 原生模式JSON文件路径           |
| --osynic-songs | -n   | -           | Osynic模式JSON文件路径         |
| --source       | -s   | SayoApi     | osu!谱面下载源                 |
| --username     | -u   | -           | osu!账号（仅OsuDirect/OsuApiV2需要） |
| --password     | -p   | -           | osu!密码（仅OsuDirect/OsuApiV2需要） |
| --output       | -o   | beatmapsets | 下载目录（自动创建）           |
| --concurrency  | -c   | 4           | 下载并发数（1-16）             |
| --help         | -h   | -           | 显示帮助信息                   |

## 支持的osu!下载源

1. **OsuDirect**：osu!官方谱面下载源（需osu账号密码）
2. **OsuApiV2**: osu!lazer的谱面下载源（需osu账号密码，做Basic认证）
3. **SayoApi**（默认）：Sayobot谱面下载源（无需登录）
4. **ChimuApi**：Chimu.moe谱面下载源（无需登录）

## 📌 注意事项

1. 视频下载适配（no_video）暂未实现，相关选项会被忽略
2. 下载文件命名遵循`{{filename}}`命名规则
3. 使用`Ctrl+C`中断下载进程后，可重新运行恢复下载
4. 建议使用稳定的网络连接以获得最佳体验

## 🤝 贡献指南

欢迎通过Issue提交建议或Pull Request参与开发！请确保：

- 遵循Rust官方编码规范
- 新增功能需附带测试用例
- 提交前运行`cargo fmt`和`cargo clippy`

## 📜 开源协议

本项目基于 [MIT License](LICENSE) 开源，请尊重原作者的著作权。使用osu!相关资源时请遵守[osu!社区准则](https://osu.ppy.sh/wiki/zh/Legal)。
