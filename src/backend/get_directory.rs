extern crate dirs;
use std::path::PathBuf;

pub fn get_video_directory() -> Option<PathBuf>{
  dirs::video_dir()
}