pub mod api {
  // import neccesary modules
  use std::io::Cursor;
  use std::path::PathBuf;

  type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
    pub async fn download_video(url: String, file_name: PathBuf) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
  }
 
}