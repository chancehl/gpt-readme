use serde::{Deserialize, Serialize};
// use std::error::Error;
use reqwest::Error;

extern crate reqwest;

#[derive(Serialize, Deserialize)]
struct GitDiff {
    diff: String,
}

#[derive(Serialize, Deserialize)]
struct ChatGPTRequest {
    git_diff: GitDiff,
}

#[derive(Serialize, Deserialize)]
struct ChatGPTResponse {
    readme: String,
}

async fn generate_readme(git_diff: GitDiff) -> Result<String, Error> {
    let api_url = "";

    let client = reqwest::Client::new();

    let request = ChatGPTRequest { git_diff };

    let response = client.post(api_url).json(&request).send().await?;

    if response.status().is_success() {
        let response_data: ChatGPTResponse = response.json().await?;

        Ok(response_data.readme)
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

#[tokio::main]
async fn main() {
    let git_diff = GitDiff {
        diff: "TODO".to_owned(),
    };

    match generate_readme(git_diff).await {
        Ok(readme) => {
            println!("{}", readme)
        }
        Err(error) => {
            eprintln!("{}", error)
        }
    }
}
