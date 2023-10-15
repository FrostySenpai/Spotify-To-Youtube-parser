#![allow(non_snake_case)]

use reqwest::{self, Url, header::HeaderMap};
use serde::Deserialize;
use wildmatch::{self, WildMatch};
use regex::Regex;
use dotenv::dotenv;

#[derive(Deserialize)]
struct YouTubeSearchResponse {
    items: Vec<YouTubeVideo>,
}

#[derive(Deserialize)]
struct YouTubeVideo {
    id: YouTubeVideoId,
}

#[derive(Deserialize)]
struct YouTubeVideoId {
    videoId: String,
}

fn main() {
    let url: Vec<_> = std::env::args().collect();
    if url.len() > 1 {
        if WildMatch::new("https://open.spotify.com/track/*").matches(&url[1]) {
            let response = request(url[1].parse::<Url>().unwrap());
            let slice = &response.unwrap()[102..180];
            let re = Regex::new(r"^>|- song and lyrics by |( \| Spot.*)").unwrap();
            let stripped = re.replace_all(slice, "");
            match yt_search(&stripped) {
                Ok(Some(video_url)) => {
                    println!("Here is your result on Youtube: {}", video_url);
                }
                Ok(None) => {
                    println!("No videos found for the search term.");
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }

        }
        if WildMatch::new("https://deezer.page.link/*").matches(&url[1]) {
            let response = request(url[1].parse::<Url>().unwrap());
            let slice = &response.unwrap()[133..200];
            let re = Regex::new(r"^>| -|(:.*)").unwrap();
            let stripped = re.replace_all(slice, "");
            match yt_search(&stripped) {
                Ok(Some(video_url)) => {
                    println!("Here is your result on Youtube: {}", video_url);
                }
                Ok(None) => {
                    println!("No videos found for the search term.");
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }
}

fn request(url_parse: Url) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("Referer", "https://spotifiy.com/".parse().unwrap());
    let client = reqwest::blocking::Client::builder()
    .default_headers(headers)
    .user_agent("curl/8.4.0")
    .redirect(reqwest::redirect::Policy::limited(4))
    .build()
    .unwrap();
    let res = client.get(url_parse)
        .send()?
        .text()?;
    return Ok(res);
}


fn yt_search(search_term: &str) -> Result<Option<String>, reqwest::Error> {

    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

    let base_url = "https://www.googleapis.com/youtube/v3/search";
    let url = format!(
        "{}?q={}&key={}&maxResults=1&part=snippet",
        base_url, search_term, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let search_results: YouTubeSearchResponse = response.json()?;

    if let Some(video) = search_results.items.first() {
        let result_id = &video.id.videoId;
        let video_url = format!("https://www.youtube.com/watch?v={}", result_id);
        Ok(Some(video_url))
    } else {
        Ok(None)
    }
}

