use tokio;
use dotenv::dotenv;
use std::env;
use slash_rs::Client;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("Could not read token");
    let client = Client::new(&token);
    let res = client.gateway(10).await.unwrap();

    println!("{:?}", &res.url);
}