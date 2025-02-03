use reqwest::Client;
use serde_json::Value;

const BEARER_TOKEN: &str = "YOUR_TWITTER_BEARER_TOKEN";

pub async fn monitor_twitter() {
    let client = Client::new();
    let url = "https://api.twitter.com/2/tweets/search/recent?query=crypto";
    
    let res = client.get(url)
        .bearer_auth(BEARER_TOKEN)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    println!("Twitter Trends: {:?}", res);
}
