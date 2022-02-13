mod backend;
use std::env;
use backend::{get_directory, fetch::api};

#[tokio::main]
async fn main() {
    let directory = get_directory::create_vid_dir().unwrap();
    if let Ok(_) = api::download_video("LINK".to_string(), directory.join("test4.mp4")).await { () }

   println!("Downloaded");
}