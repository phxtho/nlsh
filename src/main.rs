use dotenv::dotenv;
use std::env;
use reqwest;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY");
    println!("{api_key}");
    let command = env::args().nth(1).expect("no command provided");
    println!("{command}");

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
     println!("{:#?}", resp);
    Ok(())
}
