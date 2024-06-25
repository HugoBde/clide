use serde::Deserialize;
use serde_json::{from_str, json};

pub struct Client {
    client:  reqwest::blocking::Client,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Client {
        let client = reqwest::blocking::Client::new();
        return Client {
            client,
            api_key,
        };
    }

    pub fn send_api_request(&self, msg: &str) -> AnthropicResponse {
        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .body(
                json!({
                    "model": "claude-3-5-sonnet-20240620",
                    "max_tokens": 1024,
                    "messages": [
                    {"role": "user", "content":msg}
                    ]
                })
                .to_string(),
            )
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .send()
            .unwrap();

        return from_str(&response.text().unwrap()).unwrap();
    }
}

#[derive(Deserialize)]
pub struct AnthropicResponse {
    pub content: Vec<AnthropicResponseContent>,
}

#[derive(Deserialize)]
pub struct AnthropicResponseContent {
    pub text: String,
}
