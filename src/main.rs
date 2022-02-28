mod backend;
use backend::{fetch::api, get_directory};
use clap::Parser;
// CLI
#[derive(Debug, Parser)]
#[clap(
    author = "Author: github.com/alejandro0619",
    version,
    about = "Simple CLI app to download videos from Youtube"
)]
struct Args {
    /// Quality to donwload the video.
    #[clap(short, long, default_value = "auto")]
    quality: String,

    /// Youtube's video valid url.
    #[clap(short, long)]
    link: String,
}

// main
#[tokio::main]
async fn main() {
    let args = Args::parse();
    // Search and create a directory to donwload videos.
    let directory = get_directory::create_vid_dir().unwrap();
    let (keys, links, _available_q, video_id, video_title) =
        api::fetch_video_info(&args.link).await.unwrap();
    // Remove / if there's any
    let video_title = video_title.replace("/", "");
    // Get the video key depeding on the quality entered by the user
    let video_key: String = api::get_links_by_quality("720p", (keys, links));
    println!("{}", video_title);
    let download_link = api::get_download_link(video_id, video_key).await.unwrap();
    api::download_video(
        download_link,
        directory.join(&format!("{}.mp4", video_title)),
    )
    .await
    .unwrap(); // Downloads the video
}
