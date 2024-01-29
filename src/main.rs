use dotenv::dotenv;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[derive(Serialize, Deserialize)]
struct Root {
    model: String,
    messages: Vec<Message>,
    tools: Vec<Tool>,
    tool_choice: String,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Tool {
    #[serde(rename = "type")]
    tool_type: String,
    function: Function,
}

#[derive(Serialize, Deserialize)]
struct Function {
    name: String,
    description: String,
    parameters: Parameters,
}

#[derive(Serialize, Deserialize)]
struct Parameters {
    #[serde(rename = "type")]
    param_type: String,
    properties: HashMap<String, Property>,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Property {
    #[serde(rename = "type")]
    property_type: String,
    description: Option<String>,
    #[serde(rename = "enum")]
    enum_values: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let client = Client::new();

    let message = Message {
        role: "user".to_string(),
        content: "What is the weather like in Boston?".to_string(),
    };

    let function_parameters = Parameters {
        param_type: "object".to_string(),
        properties: {
            let mut properties = HashMap::new();

            properties.insert(
                "diff".to_string(),
                Property {
                    property_type: "string".to_string(),
                    description: Some(
                        "The output of the git diff command for a given repository".to_string(),
                    ),
                    enum_values: None,
                },
            );

            properties
        },
        required: vec!["diff".to_string()],
    };

    let tool_function = Function {
        name: "get_current_weather".to_string(),
        description: "Get the current weather in a given location".to_string(),
        parameters: function_parameters,
    };

    let tool = Tool {
        tool_type: "function".to_string(),
        function: tool_function,
    };

    let body = Root {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![message],
        tools: vec![tool],
        tool_choice: "auto".to_string(),
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&body)
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());

    let body = response.text().await?;

    println!("Body:\n{}", body);

    Ok(())
}
