mod backend;
use std::env;
use backend::{get_directory, fetch::api};

#[tokio::main]
async fn main() {
    let path = env::current_dir();
    let path = path.unwrap().join("dir/test.mp4");
    let _dir = if let Some(vid_dir) = get_directory::get_video_directory() {
        println!("{}", vid_dir.to_str().unwrap());
        Some(vid_dir)
       } else {
           None
       };
    api::download_video("https://dl185.y2mate.com?file=M3R4SUNiN3JsOHJ6WVo3MXN2Mlg5WVM5RkYrNHVyaHAwNFJxakZzWEZMdEFvNTVrOXEvckU5QmJKcWdCaG95dEE1Vng0enZLYU5LRU5CYkNsWmN6VUdDRXVkc3Q3WG5KK29JbFp0bHdRaDZsM3JTRmp6NWpuUkwzZTUyZk03VkdPVElwaFVNbWdRREE0T25FdFJUOHRYWC9ySExTUEh4YW9uVUdMYUdXOXBwSGlGL3VLZkw4dzVrWHFES1c3NThVaTZiSjRGZW5sdXB0NnBwbVNCbHdjY0U9".to_string(), path.to_str().unwrap()).await.unwrap();

   
}