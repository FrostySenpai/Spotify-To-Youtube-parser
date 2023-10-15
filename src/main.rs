use reqwest::{self, Url, header::HeaderMap};
use serde::{Deserialize, Serialize};
use wildmatch::{self, WildMatch};

#[derive(Serialize, Deserialize, Debug)]
struct Response {

}

fn main() {
    let url: Vec<_> = std::env::args().collect();
    if url.len() > 1 {
        println!("First argument is  {}", url[1]);
        if WildMatch::new("https://open.spotify.com/track/*").matches(&url[1]) {
            let response = request(url[1].parse::<Url>().unwrap());
            //println!("{:?}", response);
            let slice = &response.unwrap()[102..160];
            println!("{slice}");

        }
        if WildMatch::new("https://deezer.page.link/*").matches(&url[1]) {
            let response = request(url[1].parse::<Url>().unwrap());
            println!("{:?}", response);
            //let slice = &response.unwrap()[106..150];
            //println!("{slice}");
        }
    }
}

fn request(url_parse: Url) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("Referer", "https://spotifiy.com/".parse().unwrap());
    let client = reqwest::blocking::Client::builder()
    .default_headers(headers)
    .user_agent("curl/8.4.0")
    .redirect(reqwest::redirect::Policy::none())
    .build()
    .unwrap();
    let res = client.get(url_parse)
        .send()?
        .text()?;
    return Ok(res);
}
