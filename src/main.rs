use std::io::Cursor;
use std::env;
 
async fn fetch_url(url: String, file_name: &str) {
    let response = reqwest::get(url).await.unwrap();
    let mut file = std::fs::File::create(file_name).unwrap();
    let mut content =  Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut file).unwrap();
}
 
#[tokio::main]
async fn main() {
    let path = env::current_dir();
    let path = path.unwrap().join("dir/test.mp4");
   fetch_url("https://dl185.y2mate.com?file=M3R4SUNiN3JsOHJ6WVo3MXN2Mlg5WVM5RkYrNHVyaHAwNFJxakZzWEZMdEFvNTVrOXEvckU5QmJKcWdCaG95dEE1Vng0enZLYU5LRU5CYkNsWmN6VUdDRXVkc3Q3WG5KK29JbFp0bHdRaDZsM3JTRmp6NWpuUkwzZTUyZk03VkdPVElwaFVNbWdRREE0T25FdFJUOHRYWC9ySExTUEh4YW9uVUdMYUdXOXBwSGlGL3VLZkw4dzVrWHFES1c3NThVaTZiSjRGZW5sdXB0NnBwbVNCbHdjY0U9".to_string(), path.to_str().unwrap()).await;

   println!("{}", path.to_str().unwrap());
}