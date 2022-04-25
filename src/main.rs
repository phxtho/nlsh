use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY");
    println!("{api_key}");
    let command = env::args().nth(1).expect("no command provided");
    println!("{command}");
}
