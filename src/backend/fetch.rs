pub mod api {
  // import neccesary modules
  use std::{io::Cursor, fs::{self, File}};
  
  type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
    pub async fn download_video(url: String, file_name: &str) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
  }
 
}