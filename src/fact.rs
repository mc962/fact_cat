use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;
use crate::model::{RandomFact};
use notify_rust::Notification;

const SERVER_URL: &str = "https://cat-fact.herokuapp.com";

pub fn notify_fact(fact: &RandomFact) {
    Notification::new()
        .summary("Fact Cat Fact")
        .body(&fact.text)
        .show()
        .unwrap();
}

pub async fn get_random_facts(amount: i32) -> Result<Vec<RandomFact>, reqwest::Error> {
    let path = "/facts/random";
    let response = get(path).await?;

    let json_resp = if amount > 1 {
        response.json::<Vec<RandomFact>>().await?
    } else {
        let single_fact = response.json::<RandomFact>().await?;
        let mut facts: Vec<RandomFact> = vec![];
        facts.push(single_fact);
        facts
    };
    Ok(json_resp)
}

// pub async fn get_facts() -> Result<FactsResponse, reqwest::Error> {
//     let path = "/facts";
//     Ok(get(path).await?)
// }

pub async fn get(request_path: &str) -> Result<reqwest::Response, reqwest::Error> {
    let mut url = Url::parse(SERVER_URL).unwrap();
    url.set_path(request_path);

    let resp = client()
        .await?
        .get(url.as_str())
        .send()
        .await?;
    Ok(resp)
}

async fn client() -> Result<reqwest::Client, reqwest::Error> {
    let timeout = Duration::new(5, 0);
    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    reqwest::Client::builder()
        .timeout(timeout)
        .default_headers(headers)
        .build()
}