use cursive::Cursive;
use ytdlrs_lib::api::{client, downloader::DownloaderBuilder};

pub fn search(c: &mut Cursive, format: u8, url: String) -> Result<(), Box<dyn std::error::Error>> {
    let format = match format {
        1 => String::from("mp4"),
        2 => String::from("mp3"),
        _ => unreachable!("Doesn't exist that format!"),
    };

    let mut client = client::APIClientBuilder::default()
        .set_url(url)
        .set_video_type(format)
        .build()?;

    let v = client
        .fetch_video_info()?
        .get_unique_key_by_quality(String::from("720p"))?;
    let video_info = client.fetch_convert_video(&v)?;

    // We build the downloader by providing the download link and the video name. And with the method download we can download the video. (Duh)
    DownloaderBuilder::default()
        .set_url(video_info.get_download_link())
        .set_file_name(String::from("test.mp4"))
        .build()?
        .download()?;
    // c.pop_layer();
    Ok(())
}
