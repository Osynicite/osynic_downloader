use vielpork::downloader::Downloader;
use vielpork::reporters::tui::TuiReporter;
use osynic_downloader::resolver::OsuBeatmapsetResolver;
use vielpork::base::structs::DownloadOptions;
use vielpork::base::enums::DownloadResource;
use vielpork::error::Result;
use vielpork::base::structs::PathPolicy;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
    async fn main() -> Result<()> {
        
        let path_policy = PathPolicy::new()
            .with_naming("custom".to_string())
            .with_template("{{filename}}");

        let options: DownloadOptions = DownloadOptions::default()
            .with_save_path("fetch".to_string())
            .with_concurrency(4)
            .with_path_policy(path_policy);

        let downloader = Arc::new(Mutex::new(Downloader::new(options, Box::new(OsuBeatmapsetResolver::new()), Box::new(TuiReporter::new()))));

        let resources = vec![
            DownloadResource::Id("114514".to_string()),
            DownloadResource::Id("1919810".to_string()),
            DownloadResource::Id("1538879".to_string()),
            DownloadResource::Id("1001653".to_string()),
            DownloadResource::Id("1449491".to_string()),
        ];


        downloader.lock().await.start(resources).await?;


        loop{
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        

    }