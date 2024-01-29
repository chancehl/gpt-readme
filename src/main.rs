use clap::Parser;
use dotenv::dotenv;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

// This is some magic hash that is available in every git repository
// https://stackoverflow.com/a/40884093
const INITIAL_COMMIT_HASH: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

const FOOTER: &str = "Shamelessly generated with LLM labor 🦾🤖";

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

fn generate_git_diff() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("diff")
        .arg(INITIAL_COMMIT_HASH)
        .arg("HEAD")
        .arg(":!*.lock")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize .env file
    dotenv().ok();

    // Parse args
    let args = Args::parse();

    // Pull diff
    let diff = generate_git_diff().expect("Could not generate git diff");

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
        model: "gpt-4".to_string(),
        messages: vec![system_message, user_message],
    };

    // Instantiate spinner
    let mut spinner = Spinner::new(Spinners::Dots9, "Generating README.md file...".into());

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
    let mut readme_file = File::create(&outfile).expect("Could not write to README.md file");

    //  Write
    let _ = write!(readme_file, "{}\n\n{}", content, FOOTER);

    // Stop spinner
    spinner.stop();

    // Inform user of success
    println!("Wrote README to {:?}", &outfile);

    Ok(())
}
