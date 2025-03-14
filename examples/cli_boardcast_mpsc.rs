use osynic_downloader::resolver::OsuBeatmapsetResolver;
use vielpork::base::enums::{DownloadResource, ProgressEvent};
use vielpork::base::structs::DownloadOptions;
use vielpork::downloader::Downloader;
use vielpork::error::Result;
use vielpork::reporters::cli_boardcast_mpsc::CliReporterBoardcastMpsc;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    let options: DownloadOptions = DownloadOptions::default()
        .with_save_path("fetch".to_string())
        .with_concurrency(3);
    let reporter = Arc::new(CliReporterBoardcastMpsc::new(128));

    let downloader = Arc::new(Mutex::new(Downloader::new(
        options,
        Box::new(OsuBeatmapsetResolver::new()),
        Box::new((*reporter).clone()),
    )));

    let mut rx = reporter.subscribe_mpsc();

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                ProgressEvent::Start { task_id, total } => {
                    println!(
                        "Starting download of beatmapset {} with total size {}",
                        task_id, total
                    );
                }
                ProgressEvent::Update { task_id, progress } => {
                    println!(
                        "Downloading beatmapset {} - {}%",
                        task_id, progress.progress_percentage
                    );
                }

                ProgressEvent::Finish { task_id, finish } => {
                    println!("Finished downloading beatmapset {}", task_id);
                    println!("Finish type: {:?}", finish);
                }
                ProgressEvent::OperationResult {
                    operation,
                    task_id,
                    code,
                    message,
                } => {
                    println!(
                        "Operation result: {:?} - {}- {} - {}",
                        operation, task_id, code, message
                    );
                }
            }
        }
    });

    let resources = vec![
        DownloadResource::Url("https://example.com".to_string()),
        DownloadResource::Url("https://example.com".to_string()),
        DownloadResource::Url("https://example.com".to_string()),
        DownloadResource::Url("https://example.com".to_string()),
        DownloadResource::Url("https://example.com".to_string()),
        DownloadResource::Url("https://example.com".to_string()),
        DownloadResource::Url("https://example.com".to_string()),
        DownloadResource::Url("https://example.com".to_string()),
        DownloadResource::Url("https://example.com".to_string()),
    ];
    // 控制下载启停，断点续联
    let downloader_clone = Arc::clone(&downloader);
    tokio::spawn(async move {
        downloader_clone
            .lock()
            .await
            .start(resources)
            .await
            .unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    downloader.lock().await.pause().await?;
    println!("Paused");
    println!("Resuming in 2 seconds");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("Resuming");
    println!("");
    println!("");
    downloader.lock().await.resume().await?;

    // 获取任务id列表
    let tasks = downloader.lock().await.get_downloading_tasks().await;

    if tasks.is_empty() {
        println!("No tasks found");
        return Ok(());
    } else {
        println!("Tasks found");
    }

    let task_id = tasks[0].id;

    println!("Pausing task task_id in 2 seconds");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    downloader.lock().await.pause_task(task_id).await?;

    println!("Paused task task_id");
    println!("Resuming task task_id in 2 seconds");
    println!("");
    println!("");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    downloader.lock().await.resume_task(task_id).await?;
    // println!("Resumed task task_id");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("Canceling task task_id in 2 seconds");
    println!("");
    println!("");
    downloader.lock().await.cancel_task(task_id).await?;
    // println!("Canceled task task_id");

    // 单个暂停然后遇到死锁了，但是好像又好了？，挺奇怪的
    // 单个取消好像没能提前设置好状态

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    downloader.lock().await.pause().await?;
    println!("Paused");
    println!("Resuming in 2 seconds");
    println!("");
    println!("");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("Resuming");
    println!("");
    println!("");
    downloader.lock().await.resume().await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("");
    println!("");

    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");

    println!("");
    println!("");
    downloader.lock().await.stop().await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    Ok(())
}
