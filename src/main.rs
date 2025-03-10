use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;
use vielpork::{
    base::{enums::DownloadResource, structs::DownloadOptions, structs::PathPolicy},
    downloader::Downloader,
    reporters::tui::TuiReporter,
};
use osynic_downloader::resolver::OsuBeatmapsetResolver;
use osynic_downloader::sources::{DownloadSource,DownloadSourceType};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to JSON file with beatmapset IDs array
    #[arg(short,long, conflicts_with = "osynic_songs")]
    beatmapsets: Option<PathBuf>,

    /// Path to JSON file with osynic song details
    #[arg(short = 'n', long, conflicts_with = "beatmapsets")]
    osynic_songs: Option<PathBuf>,

    /// Source to use for downloading. Available sources:  OsuDirect | OsuApiV2 | SayoApi | ChimuApi
    #[arg(short,long, default_value = "SayoApi")]
    source: String,

    /// Username for source authentication
    #[arg(short, long)]
    username: Option<String>,

    /// Password for source authentication
    #[arg(short, long)]
    password: Option<String>,

    /// Output directory for downloads
    #[arg(short, long, default_value = "beatmapsets")]
    output: PathBuf,

    /// Number of concurrent downloads
    #[arg(short, long, default_value_t = 4)]
    concurrency: usize,
}

#[derive(Debug, Deserialize)]
struct BeatmapsetList {
    beatmapset_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OsynicSong {
    song_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let source = args.source;

    
    if source != "OsuDirect" && source != "OsuApiV2" && source != "SayoApi" && source != "ChimuApi" {
        return Err("Invalid source".into());
    }
    
    if  source == "OsuApiV2" {
        return Err("OsuApiV2 is not supported now.".into());
    }


    let download_source = DownloadSource::from(DownloadSourceType::from(source.as_str()));

    if download_source.requires_osu_credentials {
        if args.username.is_none() || args.password.is_none() {
            return Err("Username and password are required for this source".into());
        }
    }


    let resources = if let Some(beatmapsets_path) = &args.beatmapsets {
        // 处理普通谱面集模式
        let json_data = tokio::fs::read_to_string(beatmapsets_path).await?;
        let list: BeatmapsetList = serde_json::from_str(&json_data)?;
        list.beatmapset_ids
            .into_iter()
            .map(|id| DownloadResource::Params(vec![
                id,
                source.clone(),
                args.username.clone().unwrap_or_default(),
                args.password.clone().unwrap_or_default(),
            ]))
            .collect()
    } else if let Some(songs_path) = &args.osynic_songs {
        // 处理 osynic 歌曲模式
        let json_data = tokio::fs::read_to_string(songs_path).await?;
        let songs: Vec<OsynicSong> = serde_json::from_str(&json_data)?;
        songs
            .into_iter()
            .map(|song| {
                DownloadResource::Params(vec![
                    song.song_id.to_string(),
                    source.clone(),
                    args.username.clone().unwrap_or_default(),
                    args.password.clone().unwrap_or_default(),
                ])
            })
            .collect()
    } else {
        return Err("Either beatmapsets or osynic_songs must be provided".into());
    };

    let path_policy = PathPolicy::new()
        .with_naming("custom")
        .with_template("{{filename}}");

    let options = DownloadOptions::default()
        .with_save_path(args.output.to_string_lossy().into_owned())
        .with_concurrency(args.concurrency as u32)
        .with_path_policy(path_policy);

    let downloader = Arc::new(Mutex::new(Downloader::new(
        options,
        Box::new(OsuBeatmapsetResolver::new()),
        Box::new(TuiReporter::new()),
    )));

    downloader.lock().await.start(resources).await?;

    // 保持程序运行直到所有下载完成
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}