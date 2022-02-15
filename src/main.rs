mod backend;
use backend::{fetch::api, get_directory};

#[tokio::main]
async fn main() {
    let directory = get_directory::create_vid_dir().unwrap(); // Search and create a directory to donwload videos.
    let (keys, links, available_q, video_id) = api::fetch_video("https://youtu.be/T8CNMYdiUEE") // Example: https://youtu.be/T8CNMYdiUEE
        .await
        .unwrap();
    let video_key: String = api::get_links_by_quality("auto", (keys, links)); // Get the video key depeding on the quality entered by the user
                                                                              //if let Ok(_) = api::download_video("LINK".to_string(), directory.join("test4.mp4")).await { () } // Downloads the video
    println!("Available qualities to download {:#?}", available_q);
    println!("Video key: {}", video_key);
}
