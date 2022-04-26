// Author: alejandro0619
// github: github.com/alejandro0619/
// contact me: spaghetticodedev@gmail.com
mod backend;
use backend::{fetch::api, get_directory};
use clap::{ArgEnum, Parser};

// TODO:
// Improve the error managment


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

    /// Choose video format.
    #[clap(arg_enum)]
    format: Format
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
pub enum Format {
  Mp4,
  Mp3
}
  
// main
#[tokio::main]
async fn main() {
    // TO DOWNLOAD THE FILE
    let args = Args::parse();
    // Parse the Format enum:

    let format = match args.format {
      Format::Mp4 => "mp4",
      Format::Mp3 => "mp3",
    };
    
    // Search and create a directory to donwload videos.
    let directory = get_directory::create_vid_dir().unwrap();

    let (keys, links, _available_q, video_id, video_title) =
        api::fetch_video_info(&args.link, &format).await.unwrap();
    // Remove / if there's any
    let video_title = video_title.replace('/', "");
    // Get the video key depeding on the quality entered by the user
    let video_key: String = api::get_links_by_quality(&args.quality, (keys, links), format);
    let download_link = api::get_download_link(video_id, video_key).await.unwrap();
    api::download_video(
        download_link,
        directory.join(&format!("{}.{}", video_title, format)),
    )
    .await
    .unwrap(); // Downloads the file
}
