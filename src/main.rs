use dotenv::dotenv;
use std::env;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct CompletionResponse {
    pub choices: Vec<CompletionChoices>
}

#[derive(Deserialize)]
struct CompletionChoices {
    pub text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    // Retreive API Key from env vars
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY");

    // Parse command argument
    let user_prompt : String = env::args().nth(1).expect("no prompt provided");

    let client = reqwest::Client::new();

    let prompt: String = format!("Input: List files\nOutput: ls -l\nInput: {user_prompt}\nOutput:");

    let res = client.post("https://api.openai.com/v1/engines/code-davinci-002/completions")
        .header("Content-Type","application/json")    
        .header("Authorization", format!("Bearer {}", api_key))  
        .json(&serde_json::json!({
            "prompt": prompt,
            "max_tokens": 100,
            "temperature": 0,
            "top_p": 1,
            "n": 1,
            "stream": false,
            "logprobs": null,
            "stop": "\n"
        }))
        .send()
        .await?;
    
    if res.status() == 200 {
        let body = res.json::<CompletionResponse>().await?;
        println!("{:#?}", body.choices[0].text );
    }
     
    Ok(())
}
