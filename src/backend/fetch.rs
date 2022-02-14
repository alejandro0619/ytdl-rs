pub mod api {
  // import neccesary modules
  use std::io::Cursor;
  use std::path::PathBuf;
  use reqwest::{self, header::CONTENT_TYPE};
  use serde_json::{self, Value, Map}; 

  type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
    pub async fn download_video(url: String, file_name: PathBuf) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
  }

  pub async fn fetch_video(video_url: &str) -> Result<(Vec<String>, serde_json::Value)>{
    println!("working...");
    // Base Url to make the request.
    const URL_BASE: &str = "https://9convert.com/api/ajaxSearch/index";
    // These are the params, vt does not change as it will always download mp4
    let params = [("query", video_url), ("vt", "mp4")];
    // Create a client to make post request
    let client = reqwest::Client::new();
    let response_data = client
    .post(URL_BASE)
    .form(&params)
    .header(CONTENT_TYPE, "application/json") 
    .send()
    .await?
    .text() // Converts the response into text
    .await?;


    let response_data: Value = serde_json::from_str(response_data.as_str())?; //This will convert the text into a serde_json object
    // This vector here will store every response_data key.
    // I chose this approach because every key will always change (The keys that correspond to the different quality links).
    // I did not know a better way to do this rathen than mapping every key inside a vector.
    // !NOTE: in this vector there'll be only the keys that correspond to mp4 inner-object because
    // ! Every other key is well known (such as title, account, status and so on) at the time of developing this app.
    let mut response_data_keys: Vec<String> = Vec::new(); 

    //Loop through every quality link
    for x in  response_data["links"]["mp4"].as_object().unwrap() {
      let (key, value) = x; // key: &str, value: Value::Object
      let _value: &Map<std::string::String, Value> = value.as_object().unwrap(); // We convert Value::Object() into a real JSON
      // NOTE: I know I should use that variable above, but not yet.
      response_data_keys.push(key.to_owned());
    }
    // test if it works
    for k in &response_data_keys {
      dbg!("{}", &response_data["links"]["mp4"][k]);
      // Spoiler: Yes, it did.
    }
    //println!("{:#?}", response_data);
    Ok((response_data_keys, response_data))
  }
 
}