pub mod api {
    // import neccesary modules
    use colored::*;
    use console::Term;
    use reqwest::{self, header::CONTENT_TYPE};
    use serde_json::{self, Map, Value};
    use std::io::Cursor;
    use std::path::PathBuf;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    pub async fn download_video(url: String, file_name: PathBuf) -> Result<()> {
        let term = Term::stdout();
        println!("{}", "Downloading video... please wait".green());
        let response = reqwest::get(url).await?;
        let mut file = std::fs::File::create(file_name)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file).unwrap();
        term.clear_screen()?;
        Ok(())
    }

    ///Receives a url-compatible &string and returns a tuple with link keys and an object with those links
    pub async fn fetch_video_info(
        video_url: &str,
    ) -> Result<(Vec<String>, serde_json::Value, Vec<String>, String, String)> {
        let mut available_q: Vec<String> = Vec::new(); // This vector stores all the qualities available fo the video to download
        println!("{}", "Fetching videos".blue());
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
        for x in response_data["links"]["mp4"].as_object().unwrap() {
            let (key, value) = x; // key: &str, value: Value::Object
            let _value: &Map<std::string::String, Value> = value.as_object().unwrap(); // We convert Value::Object() into a real JSON.
                                                                                       // NOTE: I know I should use that variable above, but not yet.
            response_data_keys.push(key.to_owned());
        }

        for a in &response_data_keys {
            available_q.push(
                response_data["links"]["mp4"][a].as_object().unwrap()["q"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            );
        }
        let video_id = response_data["vid"].as_str().unwrap().to_string();
        let video_title = response_data["title"].as_str().unwrap().to_string();
        Ok((
            response_data_keys,
            response_data,
            available_q,
            video_id,
            video_title,
        ))
    }

    pub fn get_links_by_quality(
        quality: &str,
        links_tuple: (Vec<String>, serde_json::Value),
    ) -> String {
        //let quality = Value::String(quality.to_owned()); // Convert quality &str type into a a Value::String() type to use later.
        let (keys, links_obj) = links_tuple; // Destructuring th tuple.
        let mut links: Vec<&Map<std::string::String, Value>> = Vec::new(); // Creates a new vector

        for k in &keys {
            links.push(links_obj["links"]["mp4"][k].as_object().unwrap());
        }
        let mut video_key = String::new();
        for (i, _) in links.iter().enumerate() {
            if quality.to_owned() == links[i]["q"].as_str().unwrap() {
                video_key = links[i]["k"].as_str().unwrap().to_string();
            } else {
                continue;
            }
        }
        video_key
    }

    pub async fn get_download_link(video_id: String, video_key: String) -> Result<String> {
        const URL_BASE: &str = "https://9convert.com/api/ajaxConvert/convert";
        let client = reqwest::Client::new();
        let response_data = client
            .post(URL_BASE)
            .form(&[("vid", video_id), ("k", video_key)])
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?
            .text() // Converts the response into text
            .await?;
        let response_data = serde_json::from_str::<Value>(&response_data)?;

        Ok(response_data.as_object().unwrap()["dlink"]
            .as_str()
            .unwrap()
            .to_string())
    }
}
