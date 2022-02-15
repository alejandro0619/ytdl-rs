extern crate colored;
extern crate console;
extern crate dirs;

use colored::*;
use console::Term;
use std::{
    fs,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

pub fn create_vid_dir() -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let term = Term::stdout();
    match dirs::video_dir() {
        Some(dir) => {
            let vid_dir = dir.join("youtube download folder");
            if !Path::new(&vid_dir).exists() {
                // Creating a folder to save the videos in:
                {
                    // I create this scope because of readability, nothing else
                    //print in console:
                    println!("{:?} {}", vid_dir, "Folder does not exist...".red());
                    thread::sleep(Duration::from_secs(3));
                    // clears terminal screen after 3 seconds of delay
                    term.clear_screen()?;
                    println!("{} {:?}", "Creating one at".blue(), vid_dir);
                }
                fs::create_dir(&vid_dir)?;
                Ok(vid_dir)
            } else {
                println!("{}", "The download will start soon".blue());
                Ok(vid_dir)
            }
        }
        None => {
            // If video's directory doesn't exist, we create one at its path
            let vid_dir = dirs::video_dir().unwrap().join("youtube download folder");
            println!(
                "User's video folder not found, creating one at... {:?}",
                vid_dir
            );
            fs::create_dir(&vid_dir)?;
            Ok(vid_dir)
        }
    }
}
