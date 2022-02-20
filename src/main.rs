mod backend;
use backend::{fetch::api, get_directory};

#[tokio::main]
async fn main() {
    let directory = get_directory::create_vid_dir().unwrap(); // Search and create a directory to donwload videos.
    let (keys, links, available_q, video_id) = api::fetch_video("https://youtu.be/T8CNMYdiUEE") // Example: https://youtu.be/T8CNMYdiUEE
        .await
        .unwrap();
    let video_key: String = api::get_links_by_quality("auto", (keys, links)); // Get the video key depeding on the quality entered by the user
    let download_link = api::get_download_link(video_id, video_key).await.unwrap();
    if let Ok(_) = api::download_video(download_link, directory.join("test4.mp4")).await { () } // Downloads the video
    
}
