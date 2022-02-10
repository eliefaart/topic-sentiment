use crate::Result;
use reqwest::{Method, StatusCode};
use serde_derive::Deserialize;

pub struct TwitterClient {
    bearer_token: String,
    http_client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchTweetsResponse {
    data: Vec<Tweet>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tweet {
    pub id: String,
    pub text: String,
}

impl TwitterClient {
    pub fn new(bearer_token: String) -> Self {
        Self {
            bearer_token,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn get_tweets_for_topic(&self, topic: &str) -> Result<Vec<Tweet>> {
        let url =
            format!("https://api.twitter.com/2/tweets/search/recent?query={topic}&max_results=100");

        let request = self
            .http_client
            .request(Method::GET, url)
            .header("accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.bearer_token));

        let response = request.send().await?;

        if response.status() == StatusCode::OK {
            let response_body = response.json::<SearchTweetsResponse>().await?;
            Ok(response_body.data)
        } else {
            Err(Box::from(format!(
                "Non-success result on HTTP request. Status code {}.",
                response.status()
            )))
        }
    }
}
