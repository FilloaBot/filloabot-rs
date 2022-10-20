use super::main::*;

use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
pub struct Reference {
    #[serde(with = "serde_regex")]
    pub regex: Regex,
    pub message_reactions: Vec<String>,
    pub answer_text: String,
    pub answer_attachment: String,
    pub answer_reactions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    references: Vec<Reference>,
}

pub async fn get(guild_id: u64) -> Vec<Reference> {
    if API_URL.is_empty() {
        return Vec::new();
    } else {
        let client = get_client().await;

        let url = format!("{}/references/{}", *API_URL, guild_id);
        let response = client.get(url).send().await.expect("Error sending request");

        match response.status() {
            reqwest::StatusCode::OK => {
                return response.json::<ApiResponse>().await.expect("Error parsing response").references
            }
            other => {
                panic!("Unexpected response code: {:?}", other);
            }
        };
    }
}
