use dotenv::dotenv;
use std::env;
use slash_rs::Client;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let token = env::var("TOKEN").expect("Could not read token");
    let mut client = Client::build(&token).await?;

    Ok(())
}