use rusty_dl::{errors::DownloadError, youtube::YoutubeDownloader, Downloader};

pub async fn download(video_id: &str) -> Result<String, DownloadError> {
    let link = format!("https://youtu.be/{video_id}/");
    println!("Link is {}", link);
    let downloader = YoutubeDownloader::new(&link);
    match downloader?
        .with_name(video_id.to_owned())
        .only_audio() // Requires ffmpeg
        .download_to("./audio/")
        .await
    {
        Ok(_) => Ok(format!("./audio/{video_id}.mp3")),
        Err(err) => Err(err),
    }
}
