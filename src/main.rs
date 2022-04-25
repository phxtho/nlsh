use dotenv::dotenv;
use std::env;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY");
    let prompt = env::args().nth(1).expect("no prompt provided");

    let client = reqwest::Client::new();

    let res = client.post("https://api.openai.com/v1/engines/text-davinci-002/completions")
        .header("Content-Type","application/json")    
        .header("Authorization", format!("Bearer {}", api_key))  
        .json(&serde_json::json!({
            "prompt": prompt,
            "max_tokens": 5,
            "temperature": 1,
            "top_p": 1,
            "n": 1,
            "stream": false,
            "logprobs": null,
            "stop": "\n"
        }))
        .send()
        .await?;

     println!("{:#?}", res);
    Ok(())
}
