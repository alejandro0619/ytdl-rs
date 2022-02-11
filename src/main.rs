use std::io::Cursor;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
 
#[tokio::main]
async fn main() {
    fetch_url("https://georgik.rocks/wp-content/uploads/sianim.gif".to_string(), "siriel.gif".to_string()).await.unwrap();
}