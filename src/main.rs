use clap::Parser;
use dotenv::dotenv;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

/// A program to generate a README for your project with ChatGPT.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to your project
    #[arg(short, long)]
    path: PathBuf,

    /// Where to save the README file to
    #[arg(short, long)]
    out: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Root {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ChatCompletion {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    index: u8,
    message: Message,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize .env file
    dotenv().ok();

    // Parse args
    let args = Args::parse();

    // Pull diff
    let diff = fs::read_to_string("./diff.txt").expect("Could not read diff.txt file");

    // Read key
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    // Define client
    let client = Client::new();

    // Define system message
    let system_message = Message {
        role: "system".to_string(),
        content: "You are a bot who generates markdown code for README.md files based on a user-provided git diff. Please include relevant information to the project including (but not limited to) setup instructions (based on the technologies used) and installation steps. Please include links to relevant information and documentation.".to_string(),
    };

    // Define user message
    let user_message = Message {
        role: "user".to_string(),
        content: diff,
    };

    // Define body
    let body = Root {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![system_message, user_message],
    };

    // Execute http request
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&body)
        .send()
        .await?;

    // Convert body to json
    let body = response.json::<ChatCompletion>().await?;

    // Grab content
    let content = &body.choices[0].message.content;

    // Create outfile path
    let outfile = Path::join(&args.out, "README.md");

    // Create file handle
    let mut readme_file = File::create(outfile).expect("Could not write to README.md file");

    //  Write
    let _ = write!(readme_file, "{}", content);

    Ok(())
}
