mod backend;
use std::env;
use backend::{get_directory, fetch::api};

#[tokio::main]
async fn main() {
    let directory = get_directory::create_vid_dir().unwrap(); // Search and create a directory to donwload videos.
    if let Ok(_) = api::fetch_video("https://youtu.be/T8CNMYdiUEE").await { 
        println!("Worked");
     }
    //if let Ok(_) = api::download_video("LINK".to_string(), directory.join("test4.mp4")).await { () } // Downloads the video
    
   println!("Downloaded");
}