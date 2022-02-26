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
    let (keys, links, _available_q, video_id) =
        api::fetch_video(&args.link) 
            .await
            .unwrap();
    // Get the video key depeding on the quality entered by the user
    let video_key: String = api::get_links_by_quality(&args.quality, (keys, links)); 
    let download_link = api::get_download_link(video_id, video_key).await.unwrap();
    api::download_video(download_link, directory.join("test4.mp4"))
        .await
        .unwrap(); // Downloads the video
}
